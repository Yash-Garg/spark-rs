pub struct DbManager {
    pool: sqlx::PgPool,
}

impl DbManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn set_active_channel(
        &self,
        guild_id: i64,
        channel_id: i64,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            "INSERT INTO guild_channels (guild_id, channel_id)
             VALUES ($1, $2)
             ON CONFLICT (guild_id)
             DO UPDATE SET channel_id = $2",
        )
        .bind(guild_id)
        .bind(channel_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_active_channel(&self, guild_id: i64) -> Result<Option<i64>, anyhow::Error> {
        let channel_id =
            sqlx::query_scalar("SELECT channel_id FROM guild_channels WHERE guild_id = $1")
                .bind(guild_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(channel_id)
    }

    pub async fn add_compliment(
        &self,
        guild_id: i64,
        user_id: i64,
        compliment: i64,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            "INSERT INTO user_compliments (guild_id, user_id, compliment)
             VALUES ($1, $2, $3)",
        )
        .bind(guild_id)
        .bind(user_id)
        .bind(compliment)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_compliments(
        &self,
        guild_id: i64,
        user_id: i64,
    ) -> Result<Vec<i64>, anyhow::Error> {
        let compliments = sqlx::query_scalar(
            "SELECT compliment
             FROM user_compliments
             WHERE guild_id = $1 AND user_id = $2",
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(compliments)
    }
}
