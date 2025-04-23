use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let conn = Database::connect(database_url).await?;

    let _ = conn.ping().await?;

    Ok(conn)
}
