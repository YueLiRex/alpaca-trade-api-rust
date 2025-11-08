use crate::{
  client::Client,
  models::{
    Account,
    ErrorResponse,
  },
};
use anyhow::bail;

pub trait AccountApi {
  fn get_account(&self) -> impl Future<Output = anyhow::Result<Account>>;
}

impl AccountApi for Client {
  async fn get_account(&self) -> anyhow::Result<Account> {
    let url = format!("{}/v2/account", self.base_url);

    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let account = response.json::<Account>().await?;
          Ok(account)
        } else {
          let statuscode = response.status().as_u16();
          let message = response.text().await.unwrap();
          bail!(ErrorResponse::new(statuscode as u32, message))
        }
      }
      Err(error) => bail!(error),
    }
  }
}
