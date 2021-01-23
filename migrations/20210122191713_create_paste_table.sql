-- Add migration script here
CREATE TABLE IF NOT EXISTS paste (
  id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  code VARCHAR(255) UNIQUE,
  time_created TIMESTAMP NOT NULL,
  time_updated TIMESTAMP,
  data TEXT NOT NULL
);