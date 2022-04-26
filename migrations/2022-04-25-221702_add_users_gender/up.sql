-- Your SQL goes here
CREATE TYPE gender AS ENUM ('female', 'male', 'custom');
CREATE TYPE pronoun AS ENUM ('she', 'he', 'they');

ALTER TABLE users
ADD COLUMN gender gender,
ADD COLUMN pronoun pronoun,
ADD COLUMN gender_name VARCHAR(120);
