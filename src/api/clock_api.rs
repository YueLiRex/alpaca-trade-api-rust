use crate::{
  client::Client,
  models::{
    ErrorResponse,
    MarketClock,
  },
};
use anyhow::bail;

pub trait ClockApi {
  fn get_market_clock_info(&self) -> impl Future<Output = anyhow::Result<MarketClock>>;
}

impl ClockApi for Client {
  async fn get_market_clock_info(&self) -> anyhow::Result<MarketClock> {
    let url = format!("{}/v2/clock", self.base_url);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let clock = response.json::<MarketClock>().await?;
          Ok(clock)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }
}
