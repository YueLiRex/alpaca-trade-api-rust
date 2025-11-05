use crate::{
  client::Client,
  models::Account,
};

pub trait AccountApi {
  fn get_account(&self) -> impl Future<Output = anyhow::Result<Account>>;
}

impl AccountApi for Client {
  async fn get_account(&self) -> anyhow::Result<Account> {
    let url = format!("{}/v2/account", self.base_url);
    let resp = self.client.get(url).send().await?.json::<Account>().await?;
    Ok(resp)
  }
}
