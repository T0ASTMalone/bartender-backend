CREATE SEQUENCE categories_id_seq;

CREATE TABLE IF NOT EXISTS categories
(
  id          INTEGER PRIMARY KEY DEFAULT nextval('categories_id_seq'),
  name        VARCHAR(255) NOT NULL,
  description TEXT
);


ALTER TABLE todos ADD COLUMN IF NOT EXISTS category_id INTEGER;

ALTER TABLE todos 
ADD CONSTRAINT fk_todo_category 
FOREIGN KEY (category_id) REFERENCES categories (id);

INSERT INTO categories (name, description)
VALUES ('Work', 'Tasks related to work or job responsibilities'),
       ('Personal', 'Personal tasks and errands'),
       ('Health', 'Health and fitness related tasks'),
       ('Hobbies', 'Tasks related to hobbies and interests'),
       ('Education', 'Tasks related to learning and education');
