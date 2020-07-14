-- Your SQL goes here
CREATE TABLE IF NOT EXISTS post_tag_relations (
    post INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    tag INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    CONSTRAINT id PRIMARY KEY (post, tag)
);