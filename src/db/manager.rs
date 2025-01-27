pub struct DbManager {
    pool: sqlx::PgPool,
}

impl DbManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn set_active_channel(
        &self,
        guild_id: u64,
        channel_id: u64,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            "INSERT INTO guild_channels (guild_id, channel_id)
             VALUES ($1, $2)
             ON CONFLICT (guild_id)
             DO UPDATE SET channel_id = $2",
        )
        .bind(guild_id as i64)
        .bind(channel_id as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_active_channel(&self, guild_id: u64) -> Result<Option<i64>, anyhow::Error> {
        let channel_id =
            sqlx::query_scalar("SELECT channel_id FROM guild_channels WHERE guild_id = $1")
                .bind(guild_id as i64)
                .fetch_optional(&self.pool)
                .await?;

        Ok(channel_id)
    }

    pub async fn add_compliment(
        &self,
        guild_id: u64,
        user_id: u64,
        compliment: i64,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            "INSERT INTO user_compliments (guild_id, user_id, compliment)
             VALUES ($1, $2, $3)",
        )
        .bind(guild_id as i64)
        .bind(user_id as i64)
        .bind(compliment)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_compliments(
        &self,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Vec<i32>, anyhow::Error> {
        let compliments = sqlx::query_scalar(
            "SELECT compliment
             FROM user_compliments
             WHERE guild_id = $1 AND user_id = $2",
        )
        .bind(guild_id as i64)
        .bind(user_id as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(compliments)
    }
}
