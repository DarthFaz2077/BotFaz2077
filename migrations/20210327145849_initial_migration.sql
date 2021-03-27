CREATE TABLE IF NOT EXISTS public.users
(
    user_id     BIGINT      PRIMARY KEY,
    level       INT         NOT NULL DEFAULT 0,
    total_xp    INT         NOT NULL DEFAULT 0,
    current_xp  INT         NOT NULL DEFAULT 0
)

TABLESPACE pg_default;

ALTER TABLE public.users
    OWNER to postgres;
