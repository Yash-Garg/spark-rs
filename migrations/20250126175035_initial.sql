-- Create guild channels table
CREATE TABLE IF NOT EXISTS guild_channels (
    guild_id BIGINT PRIMARY KEY,
    channel_id BIGINT NOT NULL
);

-- Create user compliments table
CREATE TABLE IF NOT EXISTS user_compliments (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT REFERENCES guild_channels (guild_id),
    user_id BIGINT NOT NULL,
    compliment INTEGER NOT NULL
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS user_compliments_lookup ON user_compliments (guild_id, user_id);
