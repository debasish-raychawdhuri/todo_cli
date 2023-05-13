-- Your SQL goes here

CREATE TABLE todos (
	id char(36) PRIMARY KEY,
	description TEXT NOT NULL,
	completed BOOLEAN NOT NULL DEFAULT false
);
