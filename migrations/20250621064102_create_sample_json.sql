-- Add migration script here
-- Add migration script here
-- migrations/{20250621064102}_create_subscriptions_table.sql
-- Create Info Table
CREATE TABLE Info(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   name TEXT NOT NULL,
   email TEXT NOT NULL,
   message TEXT NOT NULL,
   sendAt timestamptz NOT NULL
);