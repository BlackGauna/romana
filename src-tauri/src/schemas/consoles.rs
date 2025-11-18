diesel::table! {
    consoles (id) {
        id -> Integer,
        name -> Text,
        abbreviation -> Text,
        manufacturer -> Text,
        in_library -> Bool,

    }
}

pub use self::consoles::dsl::*;
