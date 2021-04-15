CREATE TABLE IF NOT EXISTS guilds
(
    guild_id BIGINT NOT NULL,
    CONSTRAINT guild_pkey PRIMARY KEY (guild_id)
);

CREATE TABLE IF NOT EXISTS users_guilds
(
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    CONSTRAINT user_guild_pkey PRIMARY KEY (user_id, guild_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (user_id) MATCH SIMPLE ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fk_guild FOREIGN KEY (guild_id) REFERENCES guilds (guild_id) MATCH SIMPLE ON UPDATE CASCADE ON DELETE CASCADE
);
