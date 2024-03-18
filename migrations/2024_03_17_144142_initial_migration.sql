CREATE TYPE language AS ENUM ('RUST', 'GO', 'JAVASCRIPT', 'TYPESCRIPT', 'JAVA');

create table tests
(
    id         uuid                     default gen_random_uuid() not null,
    user_id    bigint                                             not null,
    language   language                                           not null,
    completed  boolean                  default false             not null,
    created_at timestamp with time zone default now()             not null,
    updated_at timestamp with time zone default now()             not null
);

comment on column tests.user_id is 'the discord user id';

CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON tests
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();
