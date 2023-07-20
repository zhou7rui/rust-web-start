-- Add up migration script here
CREATE TABLE users (
    id uuid not null primary key,
    name text not null,
    created_at timestamp with time zone,
    updated_at timestamp with time zone
);