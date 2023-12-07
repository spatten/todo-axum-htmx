-- Add migration script here
CREATE TABLE todos (
  id serial PRIMARY KEY,
  done boolean not null default false,
  description text not null default ''
);
