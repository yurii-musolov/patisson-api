-- TODO: Create a DBA username and password to migrate all databases.

-- For auth service
CREATE USER auth_user
WITH
  ENCRYPTED PASSWORD 'auth_pass';

CREATE DATABASE auth_db;

GRANT ALL PRIVILEGES ON DATABASE auth_db TO auth_user;

ALTER DATABASE auth_db OWNER TO auth_user;