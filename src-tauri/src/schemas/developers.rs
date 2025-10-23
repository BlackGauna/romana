diesel::table! {
    developers (id) {
        id -> Integer,
        name -> Text,
    }
}

pub use self::developers::dsl::*;
