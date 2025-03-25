-- Your SQL goes here
CREATE TABLE leg
(
    id     INTEGER PRIMARY KEY,
    set_id      INTEGER,
    status TEXT,
    FOREIGN KEY (set_id) REFERENCES dartset (id)
);
