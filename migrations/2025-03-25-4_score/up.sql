-- Your SQL goes here
CREATE TABLE IF NOT EXISTS score
(
    id          INTEGER PRIMARY KEY,
    leg_id      INTEGER,
    throw_order INTEGER,
    thrown      INTEGER,
    remaining   INTEGER,
    deleted     BOOLEAN NOT NULL CHECK (deleted in (0, 1)) DEFAULT 0,
    FOREIGN KEY (leg_id) REFERENCES leg (id)
);
