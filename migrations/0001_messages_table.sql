CREATE TABLE message (
  id BIGSERIAL PRIMARY KEY,
  content VARCHAR NOT NULL,
  user_id BIGINT NOT NULL
);

CREATE TABLE "user" (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);
