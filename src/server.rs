use entity::bowl;
use entity::customization;
use leptos::prelude::*;

#[server]
pub async fn get_bowls() -> Result<Vec<bowl::Model>, ServerFnError> {
    use crate::db::get_db;
    use sea_orm::prelude::*;
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::<DbErr>::ServerError(e.to_string()))?;
    let bowls = bowl::Entity::find()
        .all(&db)
        .await
        .map_err(|e| ServerFnError::<DbErr>::ServerError(e.to_string()))?;
    Ok(bowls)
}

#[server]
pub async fn get_customizations(
    bowl: bowl::Model,
) -> Result<Vec<customization::Model>, ServerFnError> {
    use crate::db::get_db;
    use sea_orm::prelude::*;
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::<DbErr>::ServerError(e.to_string()))?;
    let customizations = bowl
        .find_related(customization::Entity)
        .all(&db)
        .await
        .map_err(|e| ServerFnError::<DbErr>::ServerError(e.to_string()))?;
    Ok(customizations)
}
