diesel::table! {
    consoles (id) {
        id -> Integer,
        name -> Text,
        abbreviation -> Text,
        manufacturer -> Text,
    }
}

pub use self::consoles::dsl::*;
