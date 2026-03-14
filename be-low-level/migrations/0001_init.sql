CREATE TABLE IF NOT EXISTS users (
    email    TEXT PRIMARY KEY,
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS win_logs (
    id         SERIAL PRIMARY KEY,
    user_email TEXT        NOT NULL REFERENCES users (email),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_win_logs_created_at ON win_logs (created_at);

INSERT INTO users (email, password) VALUES
    ('a@gmail.com',   '1234')
ON CONFLICT DO NOTHING;
