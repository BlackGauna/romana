use diesel::prelude::*;
use serde::Serialize;

use crate::{models::Console, schemas::console_locations_table};

#[derive(Queryable, Debug, Selectable, Identifiable, PartialEq, Associations, Serialize, Clone)]
#[diesel(table_name = console_locations_table)]
#[diesel(belongs_to(Console))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConsoleLocation {
    pub id: i32,
    pub location: String,
    pub console_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = console_locations_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewConsoleLocation {
    pub location: String,
    pub console_id: i32,
}
