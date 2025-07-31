ALTER TABLE "channel_messages"
ADD "nonce" UUID NOT NULL DEFAULT gen_random_uuid();

CREATE UNIQUE INDEX "index_channel_messages_nonce" ON "channel_messages" ("nonce");
