#![allow(unused)]
#![allow(clippy::all)]

use ::diesel::prelude::*;
use serde::Serialize;

use crate::models::console::Console;
use crate::models::{Region, Rom};
use crate::schemas::rom_regions::*;

#[derive(Queryable, Debug, Selectable, Identifiable, Associations, PartialEq, Clone)]
#[diesel(belongs_to(Rom))]
#[diesel(belongs_to(Region))]
#[diesel(table_name = rom_regions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(rom_id, region_id))]
pub struct RomRegion {
    pub rom_id: i32,
    pub region_id: i32,
}
