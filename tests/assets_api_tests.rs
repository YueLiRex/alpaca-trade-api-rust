use alpaca_trade_api_rust::{
  api::{
    AssetsApi,
    AssetsQueryParameter,
    AssetsStatus,
    ComaSeparatedStrings,
  },
  prelude::{
    Client,
    enums::{
      AssetClass,
      Exchange,
      Status,
    },
  },
};
use httpmock::{
  Method::GET,
  MockServer,
};
use std::str::FromStr;
use uuid::Uuid;

extern crate alpaca_trade_api_rust;

#[tokio::test]
async fn test_get_assets_should_return_assets() {
  let mockserver = MockServer::start();

  let asset_api_mock = mockserver.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/assets")
      .query_param("status", "Active")
      .query_param("asset_class", "us_equity")
      .query_param("exchange", "NASDAQ")
      .query_param("attributes", "has_options,ipo,ptp_no_exception");
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(
        r#"[
          {
            "id": "9debbce9-2270-4e40-946a-bdea8ffc1ad3",
            "class": "us_equity",
            "exchange": "OTC",
            "symbol": "CKNHF",
            "name": "Clarkson Horace Plc Ordinary Shares",
            "status": "inactive",
            "tradable": false,
            "marginable": false,
            "maintenance_margin_requirement": 100,
            "margin_requirement_long": "100",
            "margin_requirement_short": "100",
            "shortable": false,
            "easy_to_borrow": false,
            "fractionable": false,
            "attributes": []
          },
          {
            "id": "7c6f9d92-6537-4331-8e27-e03e8fe7a29c",
            "class": "us_equity",
            "exchange": "OTC",
            "symbol": "CSAIF",
            "name": "COSAN SA INDUSTRIA E COMERCIO Ordinary Shares (Brazil)",
            "status": "inactive",
            "tradable": false,
            "marginable": false,
            "maintenance_margin_requirement": 100,
            "margin_requirement_long": "100",
            "margin_requirement_short": "100",
            "shortable": false,
            "easy_to_borrow": false,
            "fractionable": false,
            "attributes": []
          }
          ]"#,
      );
  });

  let base_url = mockserver.base_url();
  let api = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let parameter = AssetsQueryParameter {
    status: AssetsStatus::Active,
    asset_class: Some(AssetClass::UsEquity),
    exchange: Some(Exchange::NASDAQ),
    attributes: Some(ComaSeparatedStrings {
      values: vec!["has_options", "ipo", "ptp_no_exception"],
    }),
  };
  match api.get_assets(&parameter).await {
    Ok(assets) => {
      assert_eq!(assets.len(), 2);
      let first_opt = assets.first();

      match first_opt {
        Some(asset) => {
          assert!(asset.attributes.is_empty(), "attributes is not empty");
          assert_eq!(asset.margin_requirement_long, 100);
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
async fn test_get_asset_by_symbol_or_id_should_return_asset() {
  let mock_server = MockServer::start();

  let asset_mock = mock_server.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/assets/AAPL");
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(
        r#"
        {
          "id": "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415",
          "class": "us_equity",
          "exchange": "NASDAQ",
          "symbol": "AAPL",
          "name": "Apple Inc. Common Stock",
          "status": "active",
          "tradable": true,
          "marginable": true,
          "maintenance_margin_requirement": 30,
          "margin_requirement_long": "30",
          "margin_requirement_short": "30",
          "shortable": true,
          "easy_to_borrow": true,
          "fractionable": true,
          "attributes": [
            "fractional_eh_enabled",
            "has_options"
          ]
        }
      "#,
      );
  });

  let base_url = mock_server.base_url();
  let client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match client.get_asset_by_symbol_or_id("AAPL").await {
    Ok(asset) => {
      assert_eq!(
        asset.id,
        Uuid::from_str("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415").unwrap()
      );
      assert_eq!(asset.symbol, String::from_str("AAPL").unwrap());
      assert_eq!(asset.status, Status::Active);
      assert!(
        asset.attributes.len() == 2,
        "attributes length not equals to 2"
      );
    }
    Err(e) => {
      asset_mock.assert();
      panic!("API call failed: {:?}", e)
    }
  }
}

#[tokio::test]
async fn test_get_asset_by_symbol_or_id_should_return_error_response() {
  let mock_server = MockServer::start();

  let asset_mock = mock_server.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/assets/FAKE");
    then
      .status(404)
      .header("Content-Type", "application/json")
      .body(
        r#"{
            "code": 40410000,
            "message": "asset not found for FAKE"
          }"#,
      );
  });

  let base_url = mock_server.base_url();
  let client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match client.get_asset_by_symbol_or_id("FAKE").await {
    Ok(_) => {
      panic!("Expect error in this test")
    }
    Err(error) => {
      asset_mock.assert();
      assert_eq!(
        error.to_string().as_str(),
        "code: 40410000, message: \"asset not found for FAKE\""
      )
    }
  }
}
