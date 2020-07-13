CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published INTEGER NOT NULL DEFAULT 0,
    views INTEGER NOT NULL DEFAULT 0
);
INSERT INTO posts (title, body, published)
VALUES('Hello, world!', 'Hello from the database', 1);