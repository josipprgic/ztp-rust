-- Add migration script here

CREATE TYPE SubscriptionStatus AS ENUM (
    'pending_confirmation',
    'confirmed',
    'unconfirmed'
);

ALTER TABLE subscriptions 
ADD COLUMN status SubscriptionStatus NOT NULL DEFAULT 'pending_confirmation';

CREATE TABLE confirmation_tokens (
    id bigint PRIMARY KEY,
    token uuid NOT NULL,

    CONSTRAINT subscription_id_fg FOREIGN KEY (id) REFERENCES subscriptions(id) 
        ON DELETE CASCADE
);

