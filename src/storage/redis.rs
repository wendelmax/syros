use crate::Result;

#[derive(Clone)]
pub struct RedisManager {
    client: redis::Client,
}

impl RedisManager {
    pub fn new(url: &str) -> Result<Self> {
        let client =
            redis::Client::open(url).map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;
        Ok(Self { client })
    }

    pub async fn get_connection(&self) -> Result<redis::aio::Connection> {
        self.client
            .get_async_connection()
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))
    }
}
