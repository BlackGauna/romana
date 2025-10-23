pub mod consoles;
pub mod developers;
pub mod games;
pub mod regions;
pub mod rom_regions;
pub mod roms;

pub use consoles::consoles as consoles_table;
pub use developers::developers as developers_table;
pub use games::games as games_table;
pub use regions::regions as regions_table;
pub use rom_regions::rom_regions as rom_regions_table;
pub use roms::roms as roms_table;

diesel::allow_tables_to_appear_in_same_query!(
    consoles_table,
    games_table,
    developers_table,
    regions_table,
    roms_table,
    rom_regions_table
);

diesel::joinable!(rom_regions_table -> regions_table (region_id));
diesel::joinable!(rom_regions_table -> roms_table (rom_id));
diesel::joinable!(roms_table -> games_table (game_id));
diesel::joinable!(games_table -> consoles_table (console_id));
