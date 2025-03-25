-- Your SQL goes here
CREATE TABLE leg
(
    id     INTEGER PRIMARY KEY NOT NULL,
    set_id      INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT ('ONGOING') references statusType(type),
    FOREIGN KEY (set_id) REFERENCES dartset (id)
);
