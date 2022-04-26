-- Your SQL goes here
DROP TYPE gender;
DROP TYPE pronoun;

ALTER TABLE users
DROP COLUMN gender gender,
DROP COLUMN pronoun pronoun,
DROP COLUMN gender_name VARCHAR(120);
