CREATE TABLE message (
  id BIGSERIAL PRIMARY KEY,
  content VARCHAR NOT NULL,
  user_id BIGINT NOT NULL
);

CREATE TABLE "user" (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR UNIQUE NOT NULL,
  password VARCHAR NOT NULL
);

CREATE OR REPLACE FUNCTION notify_message() RETURNS trigger AS $$
DECLARE
BEGIN
    PERFORM pg_notify('channel_msg', NEW.id || ' ' || NEW.user_id || ' ' || NEW.content);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER message_notify AFTER INSERT ON message
FOR EACH ROW EXECUTE PROCEDURE notify_message();