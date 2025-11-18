diesel::table! {
    releases (id) {
        id -> Integer,
        title -> Nullable<Text>,
        game_id -> Integer,
        revision -> Integer,
        parent_id -> Nullable<Integer>,
        #[sql_name = "type"]
        r#type -> Integer,
        type_misc -> Text,
        insert_id -> Integer,
        title_non_null -> Text,
        parent_id_non_null -> Integer,
        regions_hash -> Text,
    }
}

pub use self::releases::dsl::*;
