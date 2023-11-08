CREATE TABLE "feature_flags" (
    "id" SERIAL PRIMARY KEY,
    "flag" VARCHAR(255) NOT NULL UNIQUE
);

CREATE UNIQUE INDEX "index_feature_flags" ON "feature_flags" ("id");

CREATE TABLE "user_features" (
    "user_id" INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "feature_id" INTEGER NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, feature_id)
);

CREATE UNIQUE INDEX "index_user_features_user_id_and_feature_id" ON "user_features" ("user_id", "feature_id");
CREATE INDEX "index_user_features_on_user_id" ON "user_features" ("user_id");
CREATE INDEX "index_user_features_on_feature_id" ON "user_features" ("feature_id");
