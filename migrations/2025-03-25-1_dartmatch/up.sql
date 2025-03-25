-- Your SQL goes here
CREATE TABLE dartmatch
(
    id     INTEGER PRIMARY KEY,
    status TEXT NOT NULL DEFAULT ('ONGOING') references statusType(type)
);
