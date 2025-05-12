-- For auth service
CREATE USER auth_user
WITH
    ENCRYPTED PASSWORD 'auth_pass';

CREATE DATABASE auth_db;

-- TODO: https://www.postgresql.org/docs/current/sql-grant.html
-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA schema_name
GRANT ALL PRIVILEGES ON DATABASE auth_db TO auth_user;

ALTER DATABASE auth_db OWNER TO auth_user;

-- For all services
CREATE USER dba_user
WITH
    ENCRYPTED PASSWORD 'dba_pass';

GRANT ALL PRIVILEGES ON DATABASE auth_db TO dba_user;

ALTER DATABASE auth_db OWNER TO dba_user;
