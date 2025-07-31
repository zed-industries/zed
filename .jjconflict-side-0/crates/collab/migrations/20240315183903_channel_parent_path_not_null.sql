-- Add migration script here
ALTER TABLE channels ALTER parent_path SET NOT NULL;
