
diesel::table! {
    regions (id) {
        id -> Integer,
        name -> Text,
        abbreviation -> Text,
    }
}

pub use self::regions::dsl::*;
