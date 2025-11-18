use diesel_enums::diesel_enum;
use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Serialize, EnumString, Display)]
#[diesel_enum(skip_check, table = regions, case="kebab-case", id_mapping(default))]
#[strum(ascii_case_insensitive)]
pub enum Region {
    #[db_mapping(id = 0)]
    World = 0,
    Japan,
    USA,
    Europe,
    Germany,
    Australia,
    Spain,
    France,
    Sweden,
    Italia,
    Scandinavia,
}
