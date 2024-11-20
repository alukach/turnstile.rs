CREATE SCHEMA auth;

GRANT usage ON SCHEMA api TO web_anon, web_user;

GRANT SELECT ON TABLE api.bucket TO web_user;

GRANT usage ON SCHEMA auth TO web_anon, web_user;

CREATE OR REPLACE FUNCTION auth.check_token()
  RETURNS void
  LANGUAGE plpgsql
  AS $$
BEGIN
  RAISE NOTICE 'This is a test';
  IF current_setting('request.jwt.claims', TRUE)::json ->> 'email' = 'disgruntled@mycompany.com' THEN
    RAISE insufficient_privilege
    USING hint = 'Nope, we are on to you';
  END IF;
END
$$;

