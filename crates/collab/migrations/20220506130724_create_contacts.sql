CREATE TABLE IF NOT EXISTS "contacts" (
    "id" SERIAL PRIMARY KEY,
    "requesting_user_id" INTEGER REFERENCES users (id) NOT NULL,
    "receiving_user_id" INTEGER REFERENCES users (id) NOT NULL,
    "accepted" BOOLEAN NOT NULL,
    "blocked" BOOLEAN NOT NULL
);

CREATE UNIQUE INDEX "index_org_contacts_requesting_user_id_and_receiving_user_id" ON "contacts" ("requesting_user_id", "receiving_user_id");
CREATE UNIQUE INDEX "index_org_contacts_receiving_user" ON "contacts" ("receiving_user_id");
