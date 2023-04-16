-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE cocktails DROP CONSTRAINT IF EXISTS unique_name;
ALTER TABLE cocktails ALTER COLUMN name TYPE varchar(100);
DROP EXTENSION IF EXISTS citext;
