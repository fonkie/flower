use sea_orm::{Database, DatabaseConnection, DbErr};

/// Establishes a database connection to the MySQL database
pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let conn = Database::connect(database_url).await?;
    
    // Test the connection
    let _ = conn.ping().await?;
    
    Ok(conn)
}