use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use crate::models::EntityResponse;
#[derive(Debug)]
pub struct MoySkladApiClient {
    token: String,
}
pub trait MsEntity: for<'a> Deserialize<'a> + Serialize + Clone {
    fn url() -> String;
}
impl MoySkladApiClient {
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("MS_TOKEN")?;
        Ok(Self { token })
    }
    #[instrument]
    pub async fn get_all<E>(&self) -> Result<Vec<E>>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let limit = 1000;
        let mut offset = 0;
        let mut result = Vec::new();
        loop {
            debug!("requesting with limit {limit} and offset {offset}");
            let response = client
                .get(E::url())
                .bearer_auth(&self.token)
                .query(&[(limit, offset)])
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let res: EntityResponse<E> = response.json().await?;
                    if res.rows.is_empty() {
                        debug!("rows is empty!");
                        break;
                    } else {
                        debug!("Got {} rows, extending result", res.rows.len());
                        result.extend(res.rows);
                        if let Some(size) = res.meta.size {
                            debug!("Total size -> {size}");
                            if offset > size {
                                break;
                            } else {
                                offset += limit;
                            }
                        }
                    }
                }
                _ => {
                    let err_res: serde_json::Value = response.json().await?;
                    let msg = format!("{err_res:#?}\n");
                    return Err(anyhow::Error::msg(msg));
                }
            }
        }
        Ok(result)
    }
}
