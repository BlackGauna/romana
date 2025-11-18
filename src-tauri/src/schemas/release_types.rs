diesel::table! {
    release_types (id) {
        id -> Integer,
        name -> Text,
    }
}

pub use self::release_types::dsl::*;
