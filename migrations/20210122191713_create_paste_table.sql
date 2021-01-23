-- Add migration script here
CREATE TABLE IF NOT EXISTS paste (
  id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  code VARCHAR(255) UNIQUE NOT NULL,
  time_created TIMESTAMP NULL,
  time_updated TIMESTAMP NULL,
  data TEXT NOT NULL
);