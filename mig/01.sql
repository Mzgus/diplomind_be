DROP TABLE IF EXISTS users_auth;
DROP TABLE IF EXISTS users_sheets;
DROP TABLE IF EXISTS refresh_tokens;

CREATE TABLE IF NOT EXISTS users_sheets (
    id SERIAL PRIMARY KEY,
    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL,
    type_user TEXT NOT NULL,
    profile_picture TEXT
);

CREATE TABLE IF NOT EXISTS users_auth (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    pwd TEXT NOT NULL,
    id_user_sheet INTEGER REFERENCES users_sheets(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
    token TEXT NOT NULL PRIMARY KEY,
    id_user_auth INTEGER REFERENCES users_auth(id) ON DELETE CASCADE,
    expiration_date TIMESTAMPTZ NOT NULL 
);

INSERT INTO users_sheets (id, last_name, first_name, type_user, profile_picture) 
VALUES (1, 'dupon', 'pierre', 'admin', '');

INSERT INTO users_auth (email, pwd, id_user_sheet) 
VALUES ('dupon@test.com', '$argon2id$v=19$m=19456,t=2,p=1$HlMdktaGvdvVGeYoK+XLIQ$DOjIQ7E/MWH8rIZJk1iBHB13BH2IvrvDZFKPJUBr1x8', 1);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expiration 
ON refresh_tokens(expiration_date);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_auth 
ON refresh_tokens(id_user_auth);

CREATE INDEX IF NOT EXISTS idx_users_auth_email 
ON users_auth(email);
