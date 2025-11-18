diesel::table! {
    artwork_types (id) {
        id -> Integer,
        name -> Text,
    }
}

pub use self::artwork_types::dsl::*;
