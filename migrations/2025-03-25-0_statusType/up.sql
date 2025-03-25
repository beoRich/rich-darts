-- Your SQL goes here
CREATE TABLE statusType
(
    id INTEGER PRIMARY KEY,
    type      TEXT
);

insert into statusType(type) values ('ONGOING');
insert into statusType(type) values ('FINISHED');
insert into statusType(type) values ('CANCELLED');
