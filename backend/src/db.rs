// TODO: In memory cache
use tokio_postgres::{Client, Statement};

use crate::utils::hashing_utils::hash;

const DB_SCHEMA: &str = include_str!("../sql/schema.sql");

pub struct DbStatements {
    pub insert_user: Statement,
    pub get_user_by_email: Statement,
    pub check_user_exists: Statement,
}

pub struct Db {
    pub client: Client,
    pub statements: DbStatements,
}

impl Db {
    pub async fn new() -> Result<Self, tokio_postgres::Error> {
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

        let (_, insert_user, get_user_by_email, check_user_exists) = tokio::try_join!(
            client.batch_execute(DB_SCHEMA),
            client.prepare(
                "INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4)"
            ),
            client.prepare(
                "SELECT user_id, username, email, password, is_admin FROM users WHERE email = $1"
            ),
            client.prepare("SELECT user_id FROM users WHERE email = $1"),
        )?;

        println!("Database schema applied and statements prepared!");

        let statements = DbStatements {
            insert_user,
            get_user_by_email,
            check_user_exists,
        };

        let password_bytes = hash("1234");

        client
            .execute(
                "INSERT INTO users (username, email, password, is_admin) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                &[&"admin", &"admin@gmail.com", &&password_bytes[..], &true],
            )
            .await
            .unwrap();

        Ok(Self { client, statements })
    }
}
