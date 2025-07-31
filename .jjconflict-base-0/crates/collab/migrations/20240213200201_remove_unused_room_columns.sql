-- Add migration script here
ALTER TABLE rooms DROP COLUMN enviroment;
ALTER TABLE rooms DROP COLUMN environment;
ALTER TABLE room_participants DROP COLUMN in_call;
