-- Add migration script here
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    author TEXT
);

INSERT INTO books (title, author) VALUES ('Hands-on Rust', 'Seiji Villafranca');
INSERT INTO books (title, author) VALUES ('Rust Brain Teasers', 'Andrew Huberman');