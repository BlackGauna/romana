diesel::table! {
    rom_regions (rom_id, region_id) {
        rom_id -> Integer,
        region_id -> Integer,
    }
}

pub use self::rom_regions::dsl::*;
