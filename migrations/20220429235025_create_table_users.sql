-- Add migration script here
CREATE TYPE gender AS ENUM ('female', 'male', 'custom');

CREATE TYPE pronoun AS ENUM ('he', 'she', 'they');

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(60) NOT NULL,
  last_name VARCHAR(60) NOT NULL,
  email VARCHAR(254) NOT NULL UNIQUE,
  username VARCHAR(120) NOT NULL UNIQUE,
  password_hash VARCHAR(120) NOT NULL,
  birthdate TIMESTAMPTZ NOT NULL,
  gender gender NOT NULL,
  pronoun pronoun NOT NULL,
  custom_gender VARCHAR(120),
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
