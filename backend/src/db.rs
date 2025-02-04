use ahash::RandomState;
use papaya::HashMap;
use tokio_postgres::{Client, Statement};

use crate::{
    models::clothe::{Category, Color},
    utils::hashing_utils::hash,
};

const DB_SCHEMA: &str = include_str!("../sql/schema.sql");

pub struct DbStatements {
    pub insert_user: Statement,
    pub get_user_by_email: Statement,
    pub check_user_exists: Statement,
    pub insert_outfit: Statement,
    pub get_last_outfit_by_user: Statement,
    pub get_last_clothes_by_user: Statement,
}

pub struct Db {
    pub client: Client,
    pub statements: DbStatements,
}

#[derive(serde::Serialize)]
pub struct ClotheCache {
    pub name: String,
    pub color: Color,
    pub category: Category,
    pub is_hot_weather: bool,
}

pub struct UserCache {
    pub username: String,
    pub email: String,
    pub password: [u8; 48],
    pub is_admin: bool,
    pub clothes: HashMap<i16, ClotheCache, RandomState>,
}

pub struct Cache {
    pub users: HashMap<i16, UserCache, RandomState>,
}

impl Db {
    pub async fn new() -> Result<(Self, Cache), tokio_postgres::Error> {
        let connection_str = match cfg!(feature = "docker") {
            true if cfg!(debug_assertions) => {
                "host=postgres_db_dev user=pdsdi dbname=clothe_match password=pdsdi"
            }
            true => "host=postgres_db user=pdsdi dbname=clothe_match password=pdsdi",
            false => "host=localhost user=pdsdi dbname=clothe_match password=pdsdi",
        };

        let (client, connection) = tokio_postgres::connect(connection_str, tokio_postgres::NoTls)
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
            get_user_by_email,
            check_user_exists,
            insert_outfit,
            get_last_outfit_by_user,
            get_last_clothes_by_user,
        ) = tokio::try_join!(
            client.batch_execute(DB_SCHEMA),
            client.prepare(
                "INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4)"
            ),
            client.prepare(
                "SELECT user_id, username, email, password, is_admin FROM users WHERE email = $1"
            ),
            client.prepare("SELECT user_id FROM users WHERE email = $1"),
            client.prepare("INSERT INTO outfits (name, user_id, color_mask, outfit_type) VALUES ($1, $2, $3, $4) RETURNING outfit_id"),
            client.prepare("SELECT outfit_id, name, created_at, user_id, color_mask, outfit_type FROM outfits WHERE user_id = $1 ORDER BY created_at DESC LIMIT 1"),
            client.prepare("SELECT c.clothe_id, c.name, c.color, c.category, c.user_id, c.is_for_hot_weather FROM clothes c INNER JOIN outfit_clothes oc ON c.clothe_id = oc.clothe_id WHERE oc.outfit_id = $1")



        )?;

        println!("Database schema applied and statements prepared!");

        let statements = DbStatements {
            insert_user,
            get_user_by_email,
            check_user_exists,
            insert_outfit,
            get_last_outfit_by_user,
            get_last_clothes_by_user,
        };

        let password_bytes = hash("1234");

        client
            .execute(
                "INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                &[&"admin", &"admin@gmail.com", &&password_bytes[..], &true],
            )
            .await
            .unwrap();

        let users = client
            .query(
                "SELECT user_id, username, email, password, is_admin FROM users",
                &[],
            )
            .await
            .unwrap();

        let clothes = client
            .query(
                "SELECT clothe_id, name, color, category, user_id, is_for_hot_weather FROM clothes",
                &[],
            )
            .await
            .unwrap();

        let users_cache = HashMap::builder()
            .hasher(RandomState::new())
            .capacity(users.len())
            .build();

        for user in users {
            let user_id: i16 = user.get(0);
            let username: String = user.get(1);
            let email: String = user.get(2);
            let password_slice: &[u8] = user.get(3);
            let mut password_bytes = [0u8; 48];
            password_bytes.copy_from_slice(password_slice);
            let is_admin: bool = user.get(4);

            let user_cache = UserCache {
                username,
                email,
                password: password_bytes,
                is_admin,
                clothes: HashMap::builder().hasher(RandomState::new()).build(),
            };

            for clothe in &clothes {
                let clothe_id: i16 = clothe.get(0);
                let name: String = clothe.get(1);
                let color: i16 = clothe.get(2);
                let category: i16 = clothe.get(3);
                let clothe_user_id: i16 = clothe.get(4);
                let is_hot_weather: bool = clothe.get(5);

                if user_id == clothe_user_id {
                    user_cache.clothes.pin().insert(
                        clothe_id,
                        ClotheCache {
                            name,
                            color: Color::from(color),
                            category: Category::from(category),
                            is_hot_weather,
                        },
                    );
                }
            }

            users_cache.pin().insert(user_id, user_cache);
        }

        let cache = Cache { users: users_cache };

        Ok((Self { client, statements }, cache))
    }
}
