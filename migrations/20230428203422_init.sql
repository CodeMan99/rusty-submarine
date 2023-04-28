CREATE TABLE IF NOT EXISTS public.user(
    "id" BIGSERIAL PRIMARY KEY,
    "username" text UNIQUE NOT NULL,
    "first" text,
    "last" text,
    "age" integer
);
