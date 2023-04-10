-- Add migration script here

-- We wrap the whole migration in a transaction to make sure
-- it succeeds or fails atomically.  We will discuss SQL
-- transactions in more details towards the end of the chapter.
-- 'sqlx' does not do it automatically for us.

BEGIN;
    -- Backfill `status` for historical entries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make 'status' mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;

