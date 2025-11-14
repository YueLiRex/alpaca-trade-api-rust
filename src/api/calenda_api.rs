use crate::{
  client::Client,
  models::{
    ErrorResponse,
    MarketCalendar,
  },
};
use anyhow::bail;
use chrono::NaiveDate;
use serde::Serialize;

pub trait CalendarApi {
  fn get_market_calendar_info(
    &self,
    query_parameter: &CalendarApiQueryParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<MarketCalendar>>>;
}

impl CalendarApi for Client {
  async fn get_market_calendar_info(
    &self,
    query_parameter: &CalendarApiQueryParameter,
  ) -> anyhow::Result<Vec<MarketCalendar>> {
    let url = format!("{}/v2/calendar", self.base_url);
    match self.client.get(url).query(query_parameter).send().await {
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
  pub start: Option<NaiveDate>,
  pub end: Option<NaiveDate>,
  pub date_type: Option<DateType>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DateType {
  Trading,
  Settlement,
}

#[cfg(test)]
mod tests {
  use crate::api::CalendarApiQueryParameter;
  use chrono::NaiveDate;

  #[test]
  fn test_calendar_api_query_parameter_serialize() {
    let parameters = CalendarApiQueryParameter {
      start: Some(NaiveDate::from_ymd_opt(2025, 1, 16).unwrap()),
      end: Some(NaiveDate::from_ymd_opt(2025, 1, 28).unwrap()),
      date_type: Some(super::DateType::Trading),
    };

    let parsed = serde_json::to_string(&parameters).unwrap();
    assert_eq!(
      parsed,
      r#"{"start":"2025-01-16","end":"2025-01-28","date_type":"TRADING"}"#
    )
  }
}
