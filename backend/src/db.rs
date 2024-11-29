use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: Vec<u8>,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub color_id: i32,
    pub color_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClothingItem {
    pub clothing_item_id: i32,
    pub name: String,
    pub color_id: i32,
    pub category_id: i32,
    pub user_id: i32,
    pub is_hot_weather: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Outfit {
    pub outfit_id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OutfitClothingItem {
    pub outfit_id: i32,
    pub clothing_item_id: i32,
}

pub async fn insert_user(client: &Client, user: &User) -> Result<u64, tokio_postgres::Error> {
    let stmt = client
        .prepare("INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4)")
        .await?;
    client
        .execute(
            &stmt,
            &[&user.username, &user.email, &user.password, &user.is_admin],
        )
        .await
}

pub async fn get_user_by_id(client: &Client, user_id: i32) -> Result<User, tokio_postgres::Error> {
    let stmt = client
        .prepare(
            "SELECT user_id, username, email, password, is_admin FROM users WHERE user_id = $1",
        )
        .await?;
    let row = client.query_one(&stmt, &[&user_id]).await?;
    Ok(User {
        user_id: row.get(0),
        username: row.get(1),
        email: row.get(2),
        password: row.get(3),
        is_admin: row.get(4),
    })
}

pub async fn insert_color(client: &Client, color: &Color) -> Result<u64, tokio_postgres::Error> {
    let stmt = client
        .prepare("INSERT INTO colors (color_name) VALUES ($1)")
        .await?;
    client.execute(&stmt, &[&color.color_name]).await
}

pub async fn get_color_by_id(
    client: &Client,
    color_id: i32,
) -> Result<Color, tokio_postgres::Error> {
    let stmt = client
        .prepare("SELECT color_id, color_name FROM colors WHERE color_id = $1")
        .await?;
    let row = client.query_one(&stmt, &[&color_id]).await?;
    Ok(Color {
        color_id: row.get(0),
        color_name: row.get(1),
    })
}

pub async fn insert_category(
    client: &Client,
    category: &Category,
) -> Result<u64, tokio_postgres::Error> {
    let stmt = client
        .prepare("INSERT INTO categories (category_name) VALUES ($1)")
        .await?;
    client.execute(&stmt, &[&category.category_name]).await
}

pub async fn get_category_by_id(
    client: &Client,
    category_id: i32,
) -> Result<Category, tokio_postgres::Error> {
    let stmt = client
        .prepare("SELECT category_id, category_name FROM categories WHERE category_id = $1")
        .await?;
    let row = client.query_one(&stmt, &[&category_id]).await?;
    Ok(Category {
        category_id: row.get(0),
        category_name: row.get(1),
    })
}

pub async fn insert_clothing_item(
    client: &Client,
    item: &ClothingItem,
) -> Result<u64, tokio_postgres::Error> {
    let stmt = client.prepare("INSERT INTO clothing_items (name, color_id, category_id, user_id, is_hot_weather) VALUES ($1, $2, $3, $4, $5)").await?;
    client
        .execute(
            &stmt,
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
    client: &Client,
    clothing_item_id: i32,
) -> Result<ClothingItem, tokio_postgres::Error> {
    let stmt = client.prepare("SELECT clothing_item_id, name, color_id, category_id, user_id, is_hot_weather FROM clothing_items WHERE clothing_item_id = $1").await?;
    let row = client.query_one(&stmt, &[&clothing_item_id]).await?;
    Ok(ClothingItem {
        clothing_item_id: row.get(0),
        name: row.get(1),
        color_id: row.get(2),
        category_id: row.get(3),
        user_id: row.get(4),
        is_hot_weather: row.get(5),
    })
}

pub async fn insert_outfit(client: &Client, outfit: &Outfit) -> Result<u64, tokio_postgres::Error> {
    let stmt = client
        .prepare("INSERT INTO outfits (name, user_id) VALUES ($1, $2)")
        .await?;
    client
        .execute(&stmt, &[&outfit.name, &outfit.user_id])
        .await
}

pub async fn get_outfit_by_id(
    client: &Client,
    outfit_id: i32,
) -> Result<Outfit, tokio_postgres::Error> {
    let stmt = client
        .prepare("SELECT outfit_id, name, created_at, user_id FROM outfits WHERE outfit_id = $1")
        .await?;
    let row = client.query_one(&stmt, &[&outfit_id]).await?;
    Ok(Outfit {
        outfit_id: row.get(0),
        name: row.get(1),
        created_at: row.get(2),
        user_id: row.get(3),
    })
}

pub async fn insert_outfit_clothing_item(
    client: &Client,
    outfit_id: i32,
    clothing_item_id: i32,
) -> Result<u64, tokio_postgres::Error> {
    let stmt = client
        .prepare("INSERT INTO outfit_clothing_items (outfit_id, clothing_item_id) VALUES ($1, $2)")
        .await?;
    client
        .execute(&stmt, &[&outfit_id, &clothing_item_id])
        .await
}

pub async fn get_outfit_clothing_items(
    client: &Client,
    outfit_id: i32,
) -> Result<Vec<ClothingItem>, tokio_postgres::Error> {
    let stmt = client.prepare("SELECT ci.clothing_item_id, ci.name, ci.color_id, ci.category_id, ci.user_id, ci.is_hot_weather FROM clothing_items ci JOIN outfit_clothing_items oci ON ci.clothing_item_id = oci.clothing_item_id WHERE oci.outfit_id = $1").await?;
    let rows = client.query(&stmt, &[&outfit_id]).await?;
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
