use crate::{
  client::Client,
  models::{
    ErrorResponse,
    MarketCalendar,
  },
};
use anyhow::bail;
use chrono::{
  DateTime,
  Utc,
};
use serde::Serialize;

pub trait CalendarApi {
  fn get_market_calendar_info(
    &self,
    query_parameter: CalendarApiQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<MarketCalendar>>>;
}

impl CalendarApi for Client {
  async fn get_market_calendar_info(
    &self,
    query_parameter: CalendarApiQueryParameter,
  ) -> anyhow::Result<Vec<MarketCalendar>> {
    let url = format!("{}/v2/calendar", self.base_url);
    match self.client.get(url).query(&query_parameter).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let calendar = response.json::<Vec<MarketCalendar>>().await?;
          Ok(calendar)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct CalendarApiQueryParameter {
  start: Option<DateTime<Utc>>,
  end: Option<DateTime<Utc>>,
  date_type: Option<DateType>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DateType {
  Trading,
  Settlement,
}
