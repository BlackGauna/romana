use diesel_enums::diesel_enum;
use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Serialize, EnumString, Display)]
#[diesel_enum(skip_check, table = release_types, case="kebab-case", id_mapping(default))]
#[strum(ascii_case_insensitive)]
pub enum ReleaseType {
    #[db_mapping(id = 0)]
    Custom = 0,
    Official,
    Romhack,
    Beta,
    Bootleg,
    Sample,
    VirtualConsole,
}
