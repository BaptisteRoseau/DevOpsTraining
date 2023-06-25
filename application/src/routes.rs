use crate::implementation::{create_item, delete_item, get_item, update_item};
use crate::models::Database;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

// Define the routes for CRUD operations
pub(crate) fn routes(
    database: Database,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_item_filter = warp::path!("items" / Uuid)
        .and(warp::get())
        .and(with_database(database.clone()))
        .and_then(get_item);

    let create_item_filter = warp::path!("items")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_database(database.clone()))
        .and_then(create_item);

    let update_item_filter = warp::path!("items" / Uuid)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_database(database.clone()))
        .and_then(update_item);

    let delete_item_filter = warp::path!("items" / Uuid)
        .and(warp::delete())
        .and(with_database(database))
        .and_then(delete_item);

    get_item_filter
        .or(create_item_filter)
        .or(update_item_filter)
        .or(delete_item_filter)
}

// Helper function to pass the database as a parameter
fn with_database(
    database: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || database.clone())
}
