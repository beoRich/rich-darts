-- Your SQL goes here
CREATE TABLE statusType
(
    id INTEGER PRIMARY KEY NOT NULL,
    dart_type      TEXT NOT NULL
);

insert into statusType(dart_type) values ('ONGOING');
insert into statusType(dart_type) values ('FINISHED');
insert into statusType(dart_type) values ('CANCELLED');
