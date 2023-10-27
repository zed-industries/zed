-- Add migration script here

ALTER TABLE projects
  DROP CONSTRAINT projects_room_id_fkey,
  ADD CONSTRAINT  projects_room_id_fkey
    FOREIGN KEY (room_id)
    REFERENCES rooms (id)
    ON DELETE CASCADE;
