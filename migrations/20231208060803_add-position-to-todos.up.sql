-- Add migration script here
ALTER TABLE todos
ADD COLUMN position real;
-- set the values to the current id
UPDATE todos set position=id;
-- make it not null
ALTER TABLE todos ALTER COLUMN position SET NOT NULL;
