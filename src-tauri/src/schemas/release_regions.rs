diesel::table! {
    release_regions (release_id, region_id) {
        release_id -> Integer,
        region_id -> Integer,
    }
}

pub use self::release_regions::dsl::*;
