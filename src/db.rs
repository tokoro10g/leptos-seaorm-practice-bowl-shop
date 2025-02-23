use sea_orm::prelude::*;

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let env_result = dotenvy::dotenv();
    if env_result.is_err() {
        Err(DbErr::Custom("No .env file found".to_string()))?;
    }
    Ok(sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap_or_default()).await?)
}
