-- Add migration script here
CREATE TABLE IF NOT EXISTS page_hits (
    ip inet,
    page VARCHAR (255) NOT NULL,
    status INTEGER NOT NULL
);
