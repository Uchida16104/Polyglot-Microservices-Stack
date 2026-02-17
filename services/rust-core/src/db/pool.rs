use sqlx::PgPool;
pub async fn create_pool(url: &str) -> anyhow::Result<PgPool> {
    Ok(PgPool::connect(url).await?)
}
