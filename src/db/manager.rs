pub struct DbManager {
    pool: sqlx::PgPool,
}

impl DbManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_spark(&self, user_id: i32, compliment: i32) -> Result<(), anyhow::Error> {
        sqlx::query("INSERT INTO sparks (user_id, compliment) VALUES ($1, $2)")
            .bind(user_id)
            .bind(compliment)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_sparks_count(&self, user_id: i32) -> Result<i64, anyhow::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sparks WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }
}
