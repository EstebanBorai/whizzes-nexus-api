-- Add migration script here

CREATE TYPE scope as ENUM ('public', 'private');

CREATE TABLE IF NOT EXISTS posts (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	user_id UUID NOT NULL,
	content VARCHAR(320) NOT NULL,
	scope scope NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (user_id) REFERENCES users(id)
)