-- Your SQL goes here
CREATE TABLE bird (
  id SERIAL PRIMARY KEY NOT NULL AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  scientific_name VARCHAR(255) NOT NULL,
  commonwealth_status VARCHAR(255) NOT NULL
);