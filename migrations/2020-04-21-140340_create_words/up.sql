-- Your SQL goes here
CREATE TABLE words (
  id INTEGER PRIMARY KEY NOT NULL,
  word VARCHAR NOT NULL,
  meaning VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  example TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)
