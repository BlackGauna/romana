-- This file should undo anything in `up.sql`
DROP TABLE if EXISTS developers;

DROP TABLE if EXISTS rom_regions;

DROP TABLE if EXISTS regions;

DROP TABLE if EXISTS roms;

DROP TABLE if EXISTS games;

DROP TABLE if EXISTS consoles;

PRAGMA foreign_keys = ON;
