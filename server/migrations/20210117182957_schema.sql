CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE posts (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_by INTEGER REFERENCES users(id) NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE tags (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_by INTEGER REFERENCES users(id) NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE files (
  id SERIAL NOT NULL PRIMARY KEY,
  post_id INTEGER REFERENCES posts(id) NOT NULL,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE comments (
  id SERIAL NOT NULL PRIMARY KEY,
  post_id INTEGER REFERENCES posts(id) NOT NULL,
  content TEXT NOT NULL,
  created_by INTEGER REFERENCES users(id) NOT NULL,
  created_date TIMESTAMP NOT NULL
);

CREATE TABLE images (
  id SERIAL NOT NULL PRIMARY KEY,
  post_id INTEGER REFERENCES posts(id) NOT NULL,
  name TEXT NOT NULL,
  thumbnail_url TEXT,
  full_res_url TEXT,
  created_by INTEGER REFERENCES users(id) NOT NULL,
  created_date TIMESTAMP NOT NULL
);