-- migrations/20230512030428_remove_salt_from_users.sql
ALTER TABLE users DROP COLUMN salt;
