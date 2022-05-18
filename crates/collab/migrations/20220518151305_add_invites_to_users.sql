ALTER TABLE users
ADD invite_code VARCHAR(64),
ADD invite_count INTEGER NOT NULL DEFAULT 0,
ADD inviter_id INTEGER REFERENCES users (id),
ADD created_at TIMESTAMP NOT NULL DEFAULT NOW();

CREATE UNIQUE INDEX "index_invite_code_users" ON "users" ("invite_code");
