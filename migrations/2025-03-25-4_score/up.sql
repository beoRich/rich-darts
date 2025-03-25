-- Your SQL goes here
CREATE TABLE IF NOT EXISTS score
(
    id          INTEGER PRIMARY KEY NOT NULL,
    leg_id      INTEGER NOT NULL,
    throw_order INTEGER NOT NULL,
    thrown      INTEGER NOT NULL,
    remaining   INTEGER  NOT NULL,
    deleted     BOOLEAN NOT NULL CHECK (deleted in (0, 1)) DEFAULT 0,
    FOREIGN KEY (leg_id) REFERENCES dartleg (id)
);
