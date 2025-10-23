CREATE TABLE games (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    console_id INTEGER REFERENCES consoles (id) NOT NULL DEFAULT 0
);

CREATE UNIQUE index game_title_per_console ON games (title, console_id);

CREATE TABLE regions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    abbreviation VARCHAR NOT NULL
);

CREATE TABLE developers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL
);

CREATE TABLE consoles (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL UNIQUE,
    abbreviation VARCHAR NOT NULL UNIQUE,
    manufacturer VARCHAR NOT NULL
);

INSERT INTO
    consoles (id, name, abbreviation, manufacturer)
VALUES
    (0, 'DUMMY', 'DUMMY', 'DUMMY');

CREATE TABLE roms (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    md5 VARCHAR NOT NULL,
    size INTEGER NOT NULL,
    game_id INTEGER REFERENCES games (id) NOT NULL
);

CREATE UNIQUE index rom_title_per_game ON roms (title, game_id);

CREATE TABLE rom_regions (
    rom_id INTEGER REFERENCES roms (id) NOT NULL,
    region_id INTEGER REFERENCES regions (id) NOT NULL,
    PRIMARY KEY (rom_id, region_id)
);