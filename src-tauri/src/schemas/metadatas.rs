diesel::table! {
    metadatas (id) {
        id -> Integer,
    }
}

pub use self::metadatas::dsl::*;
