use diesel::prelude::*;
use serde::Serialize;

use crate::{
    models::{Region, Release},
    schemas::release_regions_table,
};

#[derive(Queryable, Debug, Selectable, PartialEq, Associations, Serialize, Clone, Identifiable)]
#[diesel(table_name = release_regions_table)]
#[diesel(belongs_to(Release))]
#[diesel(belongs_to(Region))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(region_id, release_id))]
pub struct ReleaseRegion {
    pub region_id: Region,
    pub release_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = release_regions_table)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewReleaseRegion<'a> {
    pub region_id: &'a Region,
    pub release_id: &'a i32,
}

impl<'a> NewReleaseRegion<'a> {
    pub fn from_dat(region: &'a Region, release_id: &'a i32) -> Self {
        NewReleaseRegion {
            region_id: region,
            release_id,
        }
    }
}
