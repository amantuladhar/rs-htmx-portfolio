-- Add up migration script here
CREATE TABLE IF NOT EXISTS rs_portfolio_user (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT rs_portfolio_user_uk_username UNIQUE (username)
);

DO $$ BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            information_schema.tables
        WHERE
            table_schema = 'public'
            AND table_name = 'rs_portfolio_user'
    ) THEN
    INSERT INTO
        rs_portfolio_user (username, password)
    VALUES
        ('user1', 'password');

END IF;

END $$;