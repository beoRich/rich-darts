-- Your SQL goes here
CREATE TABLE dartset
(
    id INTEGER PRIMARY KEY,
    match_id      INTEGER,
    status TEXT NOT NULL DEFAULT ('ONGOING') references statusType(type),
    FOREIGN KEY (match_id) REFERENCES dartmatch (id)
);
