-- Your SQL goes here
CREATE TABLE statusType
(
    id INTEGER PRIMARY KEY NOT NULL,
    type      TEXT NOT NULL
);

insert into statusType(type) values ('ONGOING');
insert into statusType(type) values ('FINISHED');
insert into statusType(type) values ('CANCELLED');
