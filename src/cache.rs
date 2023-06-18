use mobc_redis::mobc::Pool;
use mobc_redis::RedisConnectionManager;
use redis::{AsyncCommands, RedisError};

use crate::errors::Error;

#[derive(Clone)]
pub struct Cache {
    pool: Pool<RedisConnectionManager>,
}

impl Cache {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Cache { pool }
    }

    async fn get_conn(
        &self,
    ) -> Result<
        mobc_redis::mobc::Connection<RedisConnectionManager>,
        mobc_redis::mobc::Error<RedisError>,
    > {
        self.pool.get().await
    }

    // FIXME: please
    pub async fn set_flag(&self, key: String, value: String) -> Result<(), Error> {
        let mut conn = match self.get_conn().await {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::Cache {
                    method_name: "get_conn".to_string(),
                    error: e.into(),
                })
            }
        };

        match conn.set_nx(key, value).await {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::Cache {
                    method_name: "set_nx".to_string(),
                    error: e.into(),
                })
            }
        };

        Ok(())
    }

    pub async fn get_flag(&self, key: String) -> Result<String, Error> {
        let mut conn = match self.get_conn().await {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::Cache {
                    method_name: "get_conn".to_string(),
                    error: e.into(),
                })
            }
        };

        let flag = match conn.get(key).await {
            Ok(flag) => flag,
            Err(e) => match e.kind() {
                redis::ErrorKind::TypeError => "".to_string(),
                _ => {
                    return Err(Error::Cache {
                        method_name: "conn.get".to_string(),
                        error: e.into(),
                    })
                }
            },
        };

        Ok(flag)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn set_flag() -> Result<(), Error> {
//         let cache = Cache::new()

//         Ok(())
//     }
// }
