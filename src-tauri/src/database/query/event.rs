use entity::{event, prelude::*};
use sea_orm::{prelude::*, InsertResult};

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> event::Model {
    match Event::find_by_id(id).one(db).await {
        Ok(event) => match event {
            Some(event) => event,
            None => panic!("No event found"),
        },
        Err(error) => panic!("{:?}", error),
    }
}

pub async fn insert_one(
    db: &DatabaseConnection,
    event: event::ActiveModel,
) -> InsertResult<event::ActiveModel> {
    Event::insert(event).exec(db).await.unwrap()
}
