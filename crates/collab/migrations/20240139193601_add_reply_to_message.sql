ALTER TABLE channel_messages ADD reply_to_message_id INTEGER DEFAULT NULL REFERENCES channel_messages (id)
