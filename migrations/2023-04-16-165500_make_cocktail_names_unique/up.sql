-- Your SQL goes here
-- CREATE EXTENSION IF NOT EXISTS citext;

-- ALTER TABLE cocktails ALTER COLUMN name TYPE citext;

ALTER TABLE cocktails DROP CONSTRAINT IF EXISTS unique_name;
ALTER TABLE cocktails ADD CONSTRAINT unique_name UNIQUE (name);


