diesel::table! {
    artworks (id) {
        id -> Integer,
        location -> Text,
        region_id -> Integer,
        type_id -> Integer,
        game_id -> Integer,
    }
}

pub use self::artworks::dsl::*;
