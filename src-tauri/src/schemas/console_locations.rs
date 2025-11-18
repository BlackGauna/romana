diesel::table! {
    console_locations (id) {
        id -> Integer,
        location -> Text,
        console_id -> Integer,
    }
}

pub use self::console_locations::dsl::*;
