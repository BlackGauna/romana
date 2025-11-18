-- This file should undo anything in `up.sql`
PRAGMA defer_foreign_keys = ON;

DROP TABLE if EXISTS roms;

DROP TABLE if EXISTS release_regions;

DROP TABLE if EXISTS releases;

DROP TABLE if EXISTS release_types;

DROP TABLE if EXISTS artwork_types;

DROP TABLE if EXISTS artworks;

DROP TABLE if EXISTS metadatas;

DROP TABLE if EXISTS regions;

DROP TABLE if EXISTS games;

DROP TABLE if EXISTS consoles;
