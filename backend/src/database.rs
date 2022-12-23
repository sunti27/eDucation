use sea_orm::{DatabaseConnection, Database};
use migration::{Migrator, MigratorTrait};

pub async fn connect() -> DatabaseConnection {
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");
    
    migrate(&conn).await;

    conn
}

async fn migrate(conn: &DatabaseConnection) {
    let recreate_db = dotenvy::var("RECREATE_DB").unwrap_or_default() == "true";

    if recreate_db {
        tracing::info!("Recreating database");
        Migrator::fresh(conn).await.expect("Failed to recreate database");
    } else {
        tracing::info!("Migrating database");
        Migrator::up(conn, None).await.expect("Failed to migrate database");
    }
}