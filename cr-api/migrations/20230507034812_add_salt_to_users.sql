-- migrations/20230507034812_add_salt_to_users.sql
ALTER TABLE users ADD COLUMN salt TEXT NOT NULL;

