use crate::models::{Database, Item};
use uuid::Uuid;
use warp::{Rejection, Reply};

/// Get an item from the database
pub(crate) async fn get_item(id: Uuid, database: Database) -> Result<impl Reply, Rejection> {
    let guard = database.read();
    match guard.get(&id) {
        Some(item) => Ok(warp::reply::json(&item)),
        None => Err(warp::reject::not_found()),
    }
}

pub(crate) async fn create_item(
    new_item: Item,
    database: Database,
) -> Result<impl Reply, Rejection> {
    let uuid = Uuid::new_v4();
    let mut guard = database.write();
    guard.insert(uuid, new_item.clone());
    Ok(warp::reply::json(&new_item))
}

pub(crate) async fn update_item(
    id: Uuid,
    updated_item: Item,
    database: Database,
) -> Result<impl Reply, Rejection> {
    let mut guard = database.write();
    match guard.get_mut(&id) {
        Some(item) => {
            item.name = updated_item.name;
            Ok::<_, warp::Rejection>(warp::reply::json(&item))
        }
        None => Err(warp::reject::not_found()),
    }
}
pub(crate) async fn delete_item(id: Uuid, database: Database) -> Result<impl Reply, Rejection> {
    let mut guard = database.write();
    match guard.remove(&id) {
        Some(_) => Ok::<_, warp::Rejection>(warp::reply()),
        None => Err(warp::reject::not_found()),
    }
}
