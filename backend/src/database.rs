use sea_orm::{DatabaseConnection, Database};

pub async fn connect() -> DatabaseConnection {
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    conn
}