use thiserror::Error;

use argon2_kdf::Hasher;
use tokio_postgres::{Client, Statement};

const DB_SCHEMA: &str = include_str!("../sql/schema.sql");

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database error")]
    DatabaseError(#[from] tokio_postgres::Error),
}

pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

pub struct Color {
    pub color_id: i32,
    pub color_name: String,
}

pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}

pub struct ClothingItem {
    pub clothing_item_id: i32,
    pub name: String,
    pub color_id: i32,
    pub category_id: i32,
    pub user_id: i32,
    pub is_hot_weather: bool,
}

pub struct Outfit {
    pub outfit_id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i32,
}

pub struct OutfitClothingItem {
    pub outfit_id: i32,
    pub clothing_item_id: i32,
}

pub struct DbStatements {
    pub insert_user: Statement,
    pub get_user_by_id: Statement,
    pub insert_color: Statement,
    pub get_color_by_id: Statement,
    pub insert_category: Statement,
    pub get_category_by_id: Statement,
    pub insert_clothing_item: Statement,
    pub get_clothing_item_by_id: Statement,
    pub insert_outfit: Statement,
    pub get_outfit_by_id: Statement,
    pub insert_outfit_clothing_item: Statement,
    pub get_outfit_clothing_items: Statement,
    pub check_user_exists: Statement,
}

pub struct DbClient {
    client: Client,
    statements: DbStatements,
}

impl DbClient {
    pub async fn new() -> Result<Self, tokio_postgres::Error> {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=pdsdi dbname=clothe_match password=pdsdi",
            tokio_postgres::NoTls,
        )
        .await
        .unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let (
            _,
            insert_user,
            get_user_by_id,
            insert_color,
            get_color_by_id,
            insert_category,
            get_category_by_id,
            insert_clothing_item,
            get_clothing_item_by_id,
            insert_outfit,
            get_outfit_by_id,
            insert_outfit_clothing_item,
            get_outfit_clothing_items,
            check_user_exists,
        ) = tokio::try_join!(
            client.batch_execute(DB_SCHEMA),
            client.prepare("INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4)"),
            client.prepare("SELECT user_id, username, email, password, is_admin FROM users WHERE user_id = $1"),
            client.prepare("INSERT INTO colors (color_name) VALUES ($1)"),
            client.prepare("SELECT color_id, color_name FROM colors WHERE color_id = $1"),
            client.prepare("INSERT INTO categories (category_name) VALUES ($1)"),
            client.prepare("SELECT category_id, category_name FROM categories WHERE category_id = $1"),
            client.prepare("INSERT INTO clothing_items (name, color_id, category_id, user_id, is_hot_weather) VALUES ($1, $2, $3, $4, $5)"),
            client.prepare("SELECT clothing_item_id, name, color_id, category_id, user_id, is_hot_weather FROM clothing_items WHERE clothing_item_id = $1"),
            client.prepare("INSERT INTO outfits (name, user_id) VALUES ($1, $2)"),
            client.prepare("SELECT outfit_id, name, created_at, user_id FROM outfits WHERE outfit_id = $1"),
            client.prepare("INSERT INTO outfit_clothing_items (outfit_id, clothing_item_id) VALUES ($1, $2)"),
            client.prepare("SELECT ci.clothing_item_id, ci.name, ci.color_id, ci.category_id, ci.user_id, ci.is_hot_weather FROM clothing_items ci JOIN outfit_clothing_items oci ON ci.clothing_item_id = oci.clothing_item_id WHERE oci.outfit_id = $1"),
            client.prepare("SELECT user_id FROM users WHERE username = $1 OR email = $2")

        )?;

        println!("Database schema applied!");

        let statements = DbStatements {
            insert_user,
            get_user_by_id,
            insert_color,
            get_color_by_id,
            insert_category,
            get_category_by_id,
            insert_clothing_item,
            get_clothing_item_by_id,
            insert_outfit,
            get_outfit_by_id,
            insert_outfit_clothing_item,
            get_outfit_clothing_items,
            check_user_exists,
        };

        Ok(Self { client, statements })
    }

    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<u64, RegistrationError> {
        let user_exists = self
            .client
            .query(&self.statements.check_user_exists, &[&username, &email])
            .await?;

        if user_exists.len() > 0 {
            return Err(RegistrationError::UserAlreadyExists);
        }

        let hash = Hasher::new()
            .algorithm(argon2_kdf::Algorithm::Argon2id)
            .salt_length(16)
            .iterations(4)
            .memory_cost_kib(65536)
            .hash_length(32)
            .threads(4)
            .hash(password.as_bytes())
            .unwrap()
            .to_string();

        Ok(self
            .insert_user(&User {
                user_id: 0,
                username: username.to_string(),
                email: email.to_string(),
                password: hash,
                is_admin: false,
            })
            .await?)
    }

    pub async fn insert_user(&self, user: &User) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(
                &self.statements.insert_user,
                &[&user.username, &user.email, &user.password, &user.is_admin],
            )
            .await
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, tokio_postgres::Error> {
        let row = self
            .client
            .query_one(&self.statements.get_user_by_id, &[&user_id])
            .await?;
        Ok(User {
            user_id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            password: row.get(3),
            is_admin: row.get(4),
        })
    }

    pub async fn insert_color(&self, color: &Color) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(&self.statements.insert_color, &[&color.color_name])
            .await
    }

    pub async fn get_color_by_id(&self, color_id: i32) -> Result<Color, tokio_postgres::Error> {
        let row = self
            .client
            .query_one(&self.statements.get_color_by_id, &[&color_id])
            .await?;
        Ok(Color {
            color_id: row.get(0),
            color_name: row.get(1),
        })
    }

    pub async fn insert_category(&self, category: &Category) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(&self.statements.insert_category, &[&category.category_name])
            .await
    }

    pub async fn get_category_by_id(
        &self,
        category_id: i32,
    ) -> Result<Category, tokio_postgres::Error> {
        let row = self
            .client
            .query_one(&self.statements.get_category_by_id, &[&category_id])
            .await?;
        Ok(Category {
            category_id: row.get(0),
            category_name: row.get(1),
        })
    }

    pub async fn insert_clothing_item(
        &self,
        item: &ClothingItem,
    ) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(
                &self.statements.insert_clothing_item,
                &[
                    &item.name,
                    &item.color_id,
                    &item.category_id,
                    &item.user_id,
                    &item.is_hot_weather,
                ],
            )
            .await
    }

    pub async fn get_clothing_item_by_id(
        &self,
        clothing_item_id: i32,
    ) -> Result<ClothingItem, tokio_postgres::Error> {
        let row = self
            .client
            .query_one(
                &self.statements.get_clothing_item_by_id,
                &[&clothing_item_id],
            )
            .await?;
        Ok(ClothingItem {
            clothing_item_id: row.get(0),
            name: row.get(1),
            color_id: row.get(2),
            category_id: row.get(3),
            user_id: row.get(4),
            is_hot_weather: row.get(5),
        })
    }

    pub async fn insert_outfit(&self, outfit: &Outfit) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(
                &self.statements.insert_outfit,
                &[&outfit.name, &outfit.user_id],
            )
            .await
    }

    pub async fn get_outfit_by_id(&self, outfit_id: i32) -> Result<Outfit, tokio_postgres::Error> {
        let row = self
            .client
            .query_one(&self.statements.get_outfit_by_id, &[&outfit_id])
            .await?;
        Ok(Outfit {
            outfit_id: row.get(0),
            name: row.get(1),
            created_at: row.get(2),
            user_id: row.get(3),
        })
    }

    pub async fn insert_outfit_clothing_item(
        &self,
        outfit_id: i32,
        clothing_item_id: i32,
    ) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(
                &self.statements.insert_outfit_clothing_item,
                &[&outfit_id, &clothing_item_id],
            )
            .await
    }

    pub async fn get_outfit_clothing_items(
        &self,
        outfit_id: i32,
    ) -> Result<Vec<ClothingItem>, tokio_postgres::Error> {
        let rows = self
            .client
            .query(&self.statements.get_outfit_clothing_items, &[&outfit_id])
            .await?;
        Ok(rows
            .iter()
            .map(|row| ClothingItem {
                clothing_item_id: row.get(0),
                name: row.get(1),
                color_id: row.get(2),
                category_id: row.get(3),
                user_id: row.get(4),
                is_hot_weather: row.get(5),
            })
            .collect())
    }
}
