-- Your SQL goes here
CREATE TABLE dartset
(
    id INTEGER PRIMARY KEY NOT NULL ,
    match_id      INTEGER NOT NULL ,
    status TEXT NOT NULL DEFAULT ('ONGOING') references statusType(type),
    FOREIGN KEY (match_id) REFERENCES dartmatch (id)
);
