use anyhow::Result;
use redis::Client;

pub fn init_redis_client(redis_url: &str) -> Result<Client> {
    let client = Client::open(redis_url)?;
    Ok(client)
}
