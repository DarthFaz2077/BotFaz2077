CREATE TABLE IF NOT EXISTS users
(
    user_id BIGINT NOT NULL,
    level INT NOT NULL DEFAULT 0,
    total_xp INT NOT NULL DEFAULT 0,
    current_xp INT NOT NULL DEFAULT 0,
    coins BIGINT NOT NULL DEFAULT 0,
    next_daily TIMESTAMP,
    CONSTRAINT user_pkey PRIMARY KEY (user_id)
)
