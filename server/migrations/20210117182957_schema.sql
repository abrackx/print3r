CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE models (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL,
  modified_date TIMESTAMP NOT NULL
);

CREATE TABLE tags (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_by INTEGER REFERENCES users(id) NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE files (
  id SERIAL NOT NULL PRIMARY KEY,
  model_id INTEGER REFERENCES models(id) NOT NULL,
  name TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL,
  modified_date TIMESTAMP NOT NULL
);

CREATE TABLE comments (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL,
  modified_date TIMESTAMP NOT NULL
);

CREATE TABLE model_comments (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  model_id INTEGER REFERENCES models(id) NOT NULL,
  UNIQUE(user_id, model_id)
);