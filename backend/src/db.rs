use tokio_postgres::{Client, Statement};

const DB_SCHEMA: &str = include_str!("../sql/schema.sql");

pub struct DbStatements {
    pub insert_user: Statement,
    pub get_user_by_email: Statement,
    pub check_user_exists: Statement,
    pub get_color_id_by_name: Statement,
    pub get_category_id_by_name: Statement,
    pub insert_clothe: Statement,
}

pub struct Db {
    pub client: Client,
    pub statements: DbStatements,
}

impl Db {
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

        let (_, insert_user, get_user_by_email, check_user_exists, get_color_id_by_name, get_category_id_by_name, insert_clothe) = tokio::try_join!(
            client.batch_execute(DB_SCHEMA),
            client.prepare(
                "INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4)"
            ),
            client.prepare(
                "SELECT user_id, username, email, password, is_admin FROM users WHERE email = $1"
            ),
            client.prepare("SELECT user_id FROM users WHERE email = $1"),
            client.prepare("SELECT color_id FROM colors WHERE color_name = $1"),
            client.prepare("SELECT category_id FROM categories WHERE category_name = $1"),
            client.prepare("INSERT INTO clothing_items (name, color_id, category_id, user_id, is_hot_weather) VALUES ($1, $2, $3, $4, $5)")
        )?;

        println!("Database schema applied and statements prepared!");

        let statements = DbStatements {
            insert_user,
            get_user_by_email,
            check_user_exists,
            get_color_id_by_name,
            get_category_id_by_name,
            insert_clothe,
        };

        Ok(Self { client, statements })
    }
}
