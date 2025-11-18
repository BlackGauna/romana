use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schemas::artworks::*;

#[derive(Queryable, Debug, Selectable, Serialize, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = artworks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ArtworkType {
    pub id: i32,
    pub location: String,
    pub region_id: i32,
    pub type_id: i32,
    pub game_id: i32,
}
