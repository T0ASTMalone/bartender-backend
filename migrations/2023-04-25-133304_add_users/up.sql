CREATE TABLE users (
  id          SERIAL       NOT NULL PRIMARY KEY,
  username    TEXT         NOT NULL UNIQUE,
  email       TEXT         NOT NULL UNIQUE,
  created_at  timestamp    default current_timestamp,
  updated_at  timestamp    default current_timestamp
);

CREATE TRIGGER create_user_timestamps BEFORE INSERT
  ON users FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_insert();

CREATE TRIGGER update_user_timestamps BEFORE UPDATE
  ON users FOR EACH ROW EXECUTE PROCEDURE
  handle_timestamps_on_row_update();
