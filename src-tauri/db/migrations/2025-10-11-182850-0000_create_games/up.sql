-- console
--  |
--  | one console has many games
--  |
--  game
--      |
--      | one game can have many releases (region, revision, beta, etc.)
--      |
--    release-------------------------------------------|
--        |                                             |
--        | one release can have many associated roms   | one release can be released in many regions
--        | (multi-rom games, patch roms, etc.)         | (e.g. Europe and Australia has same roms)
--        |                                             |
--      roms                                            regions
CREATE TABLE consoles (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL UNIQUE,
    abbreviation VARCHAR NOT NULL UNIQUE,
    manufacturer VARCHAR NOT NULL,
    in_library BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE console_locations (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    location VARCHAR UNIQUE NOT NULL,
    console_id INTEGER NOT NULL REFERENCES consoles (id)
);

CREATE TABLE games (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    console_id INTEGER NOT NULL REFERENCES consoles (id) ON DELETE CASCADE,
    metadata_id INTEGER REFERENCES metadatas (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX game_title_per_console ON games (title, console_id);

CREATE TABLE metadatas (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT);

CREATE TABLE artworks (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    location VARCHAR NOT NULL,
    region_id INTEGER NOT NULL REFERENCES regions (id),
    type_id INTEGER NOT NULL REFERENCES artwork_types (id) ON DELETE CASCADE,
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE
);

CREATE TABLE artwork_types (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, name VARCHAR NOT NULL UNIQUE);

INSERT INTO
    artwork_types (name)
VALUES
    ('box_front'),
    ('box_back'),
    ('logo'),
    ('icon'),
    ('screenshot'),
    ('fanart');

CREATE TABLE regions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    abbreviation VARCHAR NOT NULL
);

INSERT INTO
    regions (id, name, abbreviation)
VALUES
    (0, 'world', 'world'),
    (1, 'japan', 'jpn'),
    (2, 'usa', 'usa'),
    (3, 'europe', 'eur'),
    (4, 'germany', 'ger'),
    (5, 'australia', 'aus'),
    (6, 'spain', 'spa'),
    (7, 'france', 'fra'),
    (8, 'sweden', 'swe'),
    (9, 'italia', 'ita'),
    (10, 'scandinavia', 'sca');

CREATE TABLE release_types (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, name VARCHAR NOT NULL);

INSERT INTO
    release_types (id, name)
VALUES
    (0, 'custom'),
    (1, 'official'),
    (2, 'romhack'),
    (3, 'beta'),
    (4, 'bootleg'),
    (5, 'sample'),
    (6, 'virtual-console');

CREATE TABLE releases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR,
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    revision INTEGER NOT NULL DEFAULT 0,
    parent_id INTEGER REFERENCES releases (id) ON DELETE SET NULL, -- for e.g. romhacks to reference the original release,
    type INTEGER NOT NULL DEFAULT 1 REFERENCES release_types (id) ON DELETE CASCADE,
    type_misc VARCHAR NOT NULL DEFAULT "",
    insert_id INTEGER NOT NULL, -- helper field for insertion
    -- non-null generated columns to use as indexes for upsert comparisons
    title_non_null VARCHAR NOT NULL AS (COALESCE(title, '')),
    parent_id_non_null INTEGER NOT NULL AS (COALESCE(parent_id, -1)),
    -- hash of associated regions for easy comparison of duplicate releases
    regions_hash VARCHAR NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX release_title_per_game ON releases (
    title_non_null,
    game_id,
    revision,
    parent_id_non_null,
    regions_hash,
    type,
    type_misc
);

CREATE TABLE release_regions (
    release_id INTEGER NOT NULL REFERENCES releases (id) ON DELETE CASCADE,
    region_id INTEGER NOT NULL REFERENCES regions (id) ON DELETE CASCADE,
    PRIMARY KEY (release_id, region_id)
);

CREATE TABLE roms (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    md5 VARCHAR UNIQUE NOT NULL,
    crc VARCHAR UNIQUE NOT NULL,
    size INTEGER NOT NULL,
    release_id INTEGER NOT NULL REFERENCES releases (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX rom_title_per_release ON roms (title, release_id);
