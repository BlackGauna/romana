diesel::table! {
    roms (id) {
        id -> Integer,
        title -> Text,
        md5 -> Text,
        size -> Integer,
        game_id -> Integer,
    }
}

pub use self::roms::dsl::*;
