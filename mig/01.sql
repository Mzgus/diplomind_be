DROP TABLE IF EXISTS users_auth;
DROP TABLE IF EXISTS users_sheets;
DROP TABLE IF EXISTS refresh_token;

CREATE TABLE IF NOT EXISTS users_sheets (
    id INTEGER PRIMARY KEY,
    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL,
    type_user TEXT NOT NULL,
    profile_picture TEXT,
);

CREATE TABLE IF NOT EXISTS users_auth (
    id INTEGER PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    pwd TEXT NOT NULL,
    id_user_sheet INTEGER REFERENCES users_sheets(id) ON DELETE CASCADE,
);

CREATE TABLE IF NOT EXISTS refresh_token (
    token TEXT NOT NULL PRIMARY KEY,
    id_user_auth INTEGER REFERENCES users_auth(id) ON DELETE CASCADE,
    expiration_date TIMESTAMP NOT NULL 
);