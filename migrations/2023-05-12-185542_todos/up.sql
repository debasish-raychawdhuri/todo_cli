-- Your SQL goes here

CREATE TABLE todos (
	id SERIAL PRIMARY KEY,
	description TEXT NOT NULL,
	completed BOOLEAN NOT NULL DEFAULT false,
	user_id Integer NOT NULL,
	CONSTRAINT todos_customer_id
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);

