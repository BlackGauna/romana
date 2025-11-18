pub mod artwork_types;
pub mod artworks;
pub mod console_locations;
pub mod consoles;
pub mod developers;
pub mod games;
pub mod metadatas;
pub mod regions;
pub mod release_regions;
pub mod release_types;
pub mod releases;
pub mod roms;

pub use artwork_types::artwork_types as artwork_types_table;
pub use artworks::artworks as artworks_table;
pub use console_locations::console_locations as console_locations_table;
pub use consoles::consoles as consoles_table;
pub use developers::developers as developers_table;
pub use games::games as games_table;
pub use metadatas::metadatas as metadatas_table;
pub use regions::regions as regions_table;
pub use release_regions::release_regions as release_regions_table;
pub use release_types::release_types as release_types_table;
pub use releases::releases as releases_table;
pub use roms::roms as roms_table;

diesel::joinable!(artworks_table -> artwork_types_table (type_id));
diesel::joinable!(artworks_table -> games_table (game_id));
diesel::joinable!(artworks_table -> regions_table (region_id));
diesel::joinable!(console_locations_table -> consoles_table (console_id));
diesel::joinable!(games_table -> consoles_table (console_id));
diesel::joinable!(games_table -> metadatas_table (metadata_id));
diesel::joinable!(release_regions_table -> regions_table (region_id));
diesel::joinable!(release_regions_table -> releases_table (release_id));
diesel::joinable!(releases_table -> games_table (game_id));
diesel::joinable!(releases_table -> release_types_table (r#type));
diesel::joinable!(roms_table -> releases_table (release_id));

diesel::allow_tables_to_appear_in_same_query!(
    artwork_types_table,
    artworks_table,
    console_locations_table,
    consoles_table,
    games_table,
    metadatas_table,
    regions_table,
    release_regions_table,
    release_types_table,
    releases_table,
    roms_table,
);
