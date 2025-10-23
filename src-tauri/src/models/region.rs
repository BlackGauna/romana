use diesel::prelude::*;
use serde::Serialize;

use crate::schemas::regions_table;

#[derive(Queryable, Debug, Selectable, Serialize, Identifiable, PartialEq, Clone)]
#[diesel(table_name = regions_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
}
