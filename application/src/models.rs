use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// The basic items stored in the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Item {
    pub name: String,
    pub content: String,
}

/// The database containing all data accessed by the API
pub(crate) type Database = Arc<RwLock<HashMap<Uuid, Item>>>;
