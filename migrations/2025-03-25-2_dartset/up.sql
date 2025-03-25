-- Your SQL goes here
CREATE TABLE dartset
(
    id INTEGER PRIMARY KEY,
    match_id      INTEGER,
    status TEXT,
    FOREIGN KEY (match_id) REFERENCES match (id)
);
