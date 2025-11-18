use diesel::prelude::*;
use serde::Serialize;

use crate::{
    dat_parser::parser::DatRelease,
    models::{Game, ReleaseType, Rom},
    schemas::releases_table,
};

#[derive(Queryable, Debug, Selectable, Identifiable, PartialEq, Associations, Serialize, Clone)]
#[diesel(table_name = releases_table)]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Release, foreign_key = parent_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Release {
    pub id: i32,
    pub title: Option<String>,
    pub game_id: i32,
    pub revision: i32,
    pub parent_id: Option<i32>,
    pub r#type: ReleaseType,
    pub type_misc: String,
    pub insert_id: i32,
    pub title_non_null: String,
    pub parent_id_non_null: i32,
    pub regions_hash: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReleaseWithRoms {
    #[serde(flatten)]
    pub release: Release,
    pub roms: Vec<Rom>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = releases_table)]
pub struct NewRelease<'a> {
    pub title: String,
    pub game_id: &'a i32,
    pub revision: &'a i32,
    pub parent_id: Option<&'a i32>,
    pub r#type: &'a ReleaseType,
    pub type_misc: &'a String,
    pub insert_id: &'a i32,
    pub regions_hash: String,
}

impl<'a> NewRelease<'a> {
    pub fn from_dat(dat_release: &'a DatRelease, game_db_id: &'a i32) -> Self {
        // TODO: make generic from console information about possible extensions
        let name = dat_release.name.replace(".sfc", "");

        let mut regions: Vec<i32> = dat_release
            .regions
            .clone()
            .into_iter()
            .map(|r| r as i32)
            .collect();
        regions.sort();
        let regions_hash = regions
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join(",");

        NewRelease {
            title: name,
            revision: &dat_release.revision,
            parent_id: None,
            r#type: &dat_release.release_type,
            game_id: game_db_id,
            insert_id: &dat_release.insert_id,
            regions_hash,
            type_misc: &dat_release.misc,
        }
    }
}
