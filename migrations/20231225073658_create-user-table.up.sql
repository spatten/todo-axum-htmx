-- Add migration script here
CREATE TABLE users (
  id serial PRIMARY KEY,
  email varchar(255) NOT NULL,
  password_hash char(128) NOT NULL,
  salt char(128) NOT NULL
);
CREATE UNIQUE INDEX "users_email_key" ON users(email);

ALTER TABLE todos
ADD COLUMN user_id integer;
CREATE INDEX "todos_user_id_key" on todos(user_id);
-- TODO create user and set todos.user_id to it, and then make it not-null
-- UPDATE todos set user_id=1;
-- make it not null
-- ALTER TABLE todos ALTER COLUMN user_id SET NOT NULL;
