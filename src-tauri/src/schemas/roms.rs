diesel::table! {
    roms (id) {
        id -> Integer,
        title -> Text,
        md5 -> Text,
        crc -> Text,
        size -> Integer,
        release_id -> Integer,
    }
}

pub use self::roms::dsl::*;
