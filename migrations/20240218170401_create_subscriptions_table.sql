-- author: Josip Prgic

CREATE TABLE subscriptions(
    id bigserial primary key,
    email varchar(256) not null unique,
    name varchar(256) not null,
    subscribed_at timestamp not null
);


