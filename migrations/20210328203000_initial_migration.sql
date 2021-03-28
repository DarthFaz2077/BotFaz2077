CREATE TABLE IF NOT EXISTS public.users
(
    user_id BIGINT NOT NULL,
    level INT NOT NULL DEFAULT 0,
    total_xp INT NOT NULL DEFAULT 0,
    current_xp INT NOT NULL DEFAULT 0,
    coins BIGINT NOT NULL DEFAULT 0,
    daily_date DATE,
    CONSTRAINT user_pkey PRIMARY KEY (user_id)
)
