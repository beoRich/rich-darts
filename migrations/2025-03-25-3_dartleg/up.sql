-- Your SQL goes here
CREATE TABLE dartleg
(
    id     INTEGER PRIMARY KEY NOT NULL,
    set_id      INTEGER NOT NULL,
    leg_order INTEGER NOT NULL,
    start_score INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT ('ONGOING'), --references statusType(dart_type),
    FOREIGN KEY (set_id) REFERENCES dartset (id)
);
