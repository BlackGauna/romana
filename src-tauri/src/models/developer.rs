use diesel::prelude::*;
use serde::Serialize;

use crate::schemas::developers_table;

#[derive(Queryable, Debug, Selectable, Serialize, Identifiable, PartialEq)]
#[diesel(table_name = developers_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Developer {
    pub id: i32,
    pub name: String,
}
