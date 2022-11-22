use entity::event;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<event::Model>, sea_orm::DbErr> {
    event::Entity::find_by_id(id).one(db).await
}
