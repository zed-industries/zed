CREATE TABLE IF NOT EXISTS "project_activity_periods" (
    "id" SERIAL PRIMARY KEY,
    "duration_millis" INTEGER NOT NULL,
    "ended_at" TIMESTAMP NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "project_id" INTEGER
);

CREATE INDEX "index_project_activity_periods_on_ended_at" ON "project_activity_periods" ("ended_at");
