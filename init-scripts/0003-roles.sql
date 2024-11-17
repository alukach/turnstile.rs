CREATE ROLE web_anon nologin;

GRANT usage ON SCHEMA api TO web_anon;

GRANT SELECT ON api.bucket TO web_anon;

CREATE ROLE authenticator noinherit LOGIN PASSWORD 'mysecretpassword';

GRANT web_anon TO authenticator;

