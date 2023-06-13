-- Your SQL goes here
create table animals
(
    id            serial primary key,
    species       varchar not null,
    common_name   varchar not null,
    habitat       varchar not null,
    lifespan      integer     not null,
    is_endangered boolean not null default false
);