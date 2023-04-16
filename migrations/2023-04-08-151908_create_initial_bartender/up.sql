CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION handle_timestamps_on_row_update()
  RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now() AT TIME ZONE 'utc';
  NEW.created_at = OLD.created_at;

  RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE FUNCTION handle_timestamps_on_row_insert()
  RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = now() AT TIME ZONE 'utc';
  NEW.updated_at = NEW.created_at;
  return NEW;
END;
$$ language 'plpgsql';

-- Your SQL goes here
-- CREATE SCHEMA 
-- ALTER SCHEMA owner to postgres;

CREATE TABLE cocktails
(
  id         uuid         not null default gen_random_uuid() primary key,
  name       varchar(100) not null,
  created_at timestamp    default current_timestamp,
  updated_at timestamp    default current_timestamp
);

CREATE TRIGGER create_cocktail_timestamps BEFORE INSERT
  ON cocktails FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_insert();

CREATE TRIGGER update_cocktail_timestamps BEFORE UPDATE
  ON cocktails FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_update();

CREATE TABLE ingredients
(
  id          uuid         not null default gen_random_uuid() primary key,
  name        varchar(100) not null,
  measurement varchar(50)  not null,
  cocktail_id uuid         not null references cocktails (id) on delete cascade,
  created_at  timestamp    default current_timestamp,
  updated_at  timestamp    default current_timestamp
);

CREATE TRIGGER create_ingredient_timestamps BEFORE INSERT
  ON ingredients FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_insert();

CREATE TRIGGER update_ingredient_timestamps BEFORE UPDATE
  ON ingredients FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_update();

CREATE TABLE instructions
(
  id          uuid         not null default gen_random_uuid() primary key,
  -- confirm size with examples in notes
  instruction varchar(500) not null,
  step        smallint     not null,
  cocktail_id uuid         not null references cocktails (id) on delete cascade,
  created_at  timestamp    default current_timestamp,
  updated_at  timestamp    default current_timestamp
);

CREATE TRIGGER create_instruction_timestamps BEFORE INSERT
  ON instructions FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_insert();

CREATE TRIGGER update_instructions_timestamps BEFORE UPDATE
  ON instructions FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_update();
