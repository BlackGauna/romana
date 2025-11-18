use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schemas::artwork_types::*;

#[derive(Queryable, Debug, Selectable, Serialize, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = artwork_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ArtworkType {
    pub id: i32,
    pub name: String,
}
