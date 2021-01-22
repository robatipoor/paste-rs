-- Add migration script here
CREATE TABLE IF NOT EXISTS paste (
id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
code TEXT,
time_created TIMESTAMP,
data BLOB)