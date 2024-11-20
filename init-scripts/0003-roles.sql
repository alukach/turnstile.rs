-- Authenticator
CREATE ROLE authenticator NOINHERIT LOGIN PASSWORD 'mysecretpassword' NOCREATEDB NOCREATEROLE NOSUPERUSER;

-- Anon
CREATE ROLE web_anon nologin;

GRANT web_anon TO authenticator;

-- User
CREATE ROLE web_user NOLOGIN;

GRANT web_user TO authenticator;

