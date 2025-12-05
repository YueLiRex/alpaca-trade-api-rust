use alpaca_trade_api_rust::{
  api::{
    ComaSeparatedStrings,
    CorporateActionApi,
    CorporateActionsQueryParameter,
  },
  prelude::Client,
};
use chrono::prelude::*;
use httpmock::{
  Method::GET,
  MockServer,
};
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_get_corporate_actions_should_return_actions() {
  let mockserver = MockServer::start();

  let asset_api_mock = mockserver.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/corporate_actions/announcements");
    then.status(200).header("Content-Type", "application/json").body(
      r#"[
            {
              "id": "d99eb57c-19b4-40b9-ab1e-a0971bf5f288",
              "corporate_action_id": "2829511",
              "ca_type": "merger",
              "ca_sub_type": "merger_completion",
              "initiating_symbol": "CAC",
              "initiating_original_cusip": "133034108",
              "target_symbol": "NWYF",
              "target_original_cusip": "667270102",
              "effective_date": "2025-01-02",
              "record_date": null,
              "cash": "0",
              "old_rate": "1",
              "new_rate": "0.83"
            },
            {
              "id": "b12efdfc-7dcd-4654-813f-8519d0d333e5",
              "corporate_action_id": "2829479",
              "ca_type": "merger",
              "ca_sub_type": "merger_completion",
              "initiating_symbol": "",
              "initiating_original_cusip": "",
              "target_symbol": "MARXR",
              "target_original_cusip": "G5870E124",
              "effective_date": "2025-01-02",
              "record_date": null,
              "cash": "0",
              "old_rate": "1",
              "new_rate": "0.2"
            },
            {
              "id": "15532d5e-6901-4388-9bed-9f52f2404259",
              "corporate_action_id": "2829477",
              "ca_type": "merger",
              "ca_sub_type": "merger_completion",
              "initiating_symbol": "",
              "initiating_original_cusip": "",
              "target_symbol": "MARX",
              "target_original_cusip": "G5870E108",
              "effective_date": "2025-01-02",
              "record_date": null,
              "cash": "0",
              "old_rate": "1",
              "new_rate": "1"
            }
          ]"#,
    );
  });

  let base_url = mockserver.base_url();
  let api = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let parameter = &CorporateActionsQueryParameter {
    ca_types: ComaSeparatedStrings {
      values: vec!["dividend", "merger"],
    },
    since: NaiveDate::from_ymd_opt(2025, 1, 30).unwrap(),
    until: NaiveDate::from_ymd_opt(2025, 3, 30).unwrap(),
    symbols: None,
    cusip: None,
    date_type: None,
  };

  match api.get_corporate_actions(parameter).await {
    Ok(actions) => {
      assert_eq!(actions.len(), 3);
      let first_opt = actions.first();

      match first_opt {
        Some(action) => {
          assert_eq!(action.effective_date, Some(NaiveDate::from_str("2025-01-02").unwrap()))
        }
        None => panic!("Expect an asset, but None returned."),
      }
    }
    Err(e) => {
      asset_api_mock.assert();
      panic!("API call failed: {:?}", e)
    }
  }
}

#[tokio::test]
async fn test_get_specific_corporate_actions_should_return_action() {
  let mockserver = MockServer::start();

  let asset_api_mock = mockserver.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/corporate_actions/announcements/d99eb57c-19b4-40b9-ab1e-a0971bf5f288");
    then.status(200).header("Content-Type", "application/json").body(
      r#"{
          "id": "d99eb57c-19b4-40b9-ab1e-a0971bf5f288",
          "corporate_action_id": "2829511",
          "ca_type": "merger",
          "ca_sub_type": "merger_completion",
          "initiating_symbol": "CAC",
          "initiating_original_cusip": "133034108",
          "target_symbol": "NWYF",
          "target_original_cusip": "667270102",
          "effective_date": "2025-01-02",
          "record_date": null,
          "cash": "0",
          "old_rate": "1",
          "new_rate": "0.83"
        }"#,
    );
  });

  let base_url = mockserver.base_url();
  let api = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api
    .get_specific_corporate_actions(&Uuid::from_str("d99eb57c-19b4-40b9-ab1e-a0971bf5f288").unwrap())
    .await
  {
    Ok(action) => {
      assert_eq!(action.effective_date, Some(NaiveDate::from_str("2025-01-02").unwrap()))
    }
    Err(e) => {
      asset_api_mock.assert();
      panic!("API call failed: {:?}", e)
    }
  }
}

#[tokio::test]
async fn test_get_specific_corporate_actions_should_return_error_response() {
  let mockserver = MockServer::start();

  let asset_api_mock = mockserver.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/corporate_actions/announcements/d99eb57c-19b4-40b9-ab1e-a0971bf5f288");
    then.status(404).header("Content-Type", "application/json").body(
      r#"{
          "code": 40410000,
          "message": "resource not found"
        }"#,
    );
  });

  let base_url = mockserver.base_url();
  let api = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api
    .get_specific_corporate_actions(&Uuid::from_str("d99eb57c-19b4-40b9-ab1e-a0971bf5f288").unwrap())
    .await
  {
    Ok(_) => {
      panic!("Expect error reponse in this test case")
    }
    Err(e) => {
      asset_api_mock.assert();
      assert_eq!(
        e.to_string().as_str(),
        r#"code: 40410000, message: "resource not found""#
      )
    }
  }
}
