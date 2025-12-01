use alpaca_trade_api_rust::{
  api::{
    AddAssetReqBody,
    WatchListApi,
    WatchListReqBody,
  },
  prelude::Client,
};
use chrono::serde::ts_seconds;
use httpmock::{
  Method::{
    DELETE,
    GET,
    POST,
    PUT,
  },
  MockServer,
};
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_get_all_watch_lists_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  [
    {
      "id": "3174d6df-7726-44b4-a5bd-7fda5ae6e009",
      "account_id": "abe25343-a7ba-4255-bdeb-f7e013e9ee5d",
      "created_at": "2022-01-31T21:49:05.14628Z",
      "updated_at": "2022-01-31T21:49:05.14628Z",
      "name": "Primary Watchlist"
    }
  ]
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client.get_all_watch_lists().await {
    Ok(watchlist_info) => {
      assert_eq!(watchlist_info.len(), 1)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_create_watch_list_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-01T19:35:14.803Z",
    "updated_at": "2025-12-01T19:35:14.803Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "124132",
        "margin_requirement_short": "2141324",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(POST)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let request_body = WatchListReqBody {
    name: "test".to_string(),
    symbols: vec!["META".to_string(), "GOGL".to_string()],
  };

  match api_client.create_watch_list(request_body).await {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      );
      assert_eq!(watchlist.assets.len(), 1)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_get_watch_list_by_id_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "234",
        "margin_requirement_short": "234234",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists/3fa85f64-5717-4562-b3fc-2c963f66afa6");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client
    .get_watch_list_by_id(Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap())
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_update_watch_list_by_id_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "2142314",
        "margin_requirement_short": "123",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(PUT)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists/3fa85f64-5717-4562-b3fc-2c963f66afa6");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let req_body = WatchListReqBody {
    name: "test-watch-list".to_string(),
    symbols: vec!["META".to_string()],
  };

  match api_client
    .update_watch_list_by_id(
      Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
      req_body,
    )
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_delete_watch_list_by_id_should_return_ok() {
  let ms = MockServer::start();

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(DELETE)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists/3fa85f64-5717-4562-b3fc-2c963f66afa6");

    then.status(200).header("Content-Type", "application/json");
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client
    .delete_watch_list_by_id(Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap())
    .await
  {
    Ok(_) => {
      endpoint_mock.assert();
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_add_asset_to_watch_list_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "2134",
        "margin_requirement_short": "1234",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(POST)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists/3fa85f64-5717-4562-b3fc-2c963f66afa6");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let req_body = AddAssetReqBody {
    symbol: "META".to_string(),
  };

  match api_client
    .add_asset_to_watch_list(
      Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
      req_body,
    )
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_get_watch_list_by_name_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "234",
        "margin_requirement_short": "234234",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists:by_name")
      .query_param("name", "test-name");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client
    .get_watch_list_by_name("test-name".to_string())
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_update_watch_list_by_name_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "2142314",
        "margin_requirement_short": "123",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(PUT)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists:by_name")
      .query_param("name", "test-name");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let req_body = WatchListReqBody {
    name: "test-watch-list".to_string(),
    symbols: vec!["META".to_string()],
  };

  match api_client
    .update_watch_list_by_name("test-name".to_string(), req_body)
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_add_asset_to_watch_list_by_name_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "2134",
        "margin_requirement_short": "1234",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(POST)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists:by_name")
      .query_param("name", "test-name");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  let req_body = AddAssetReqBody {
    symbol: "META".to_string(),
  };

  match api_client
    .add_asset_to_watch_list_by_name("test-name".to_string(), req_body)
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_delete_watch_list_by_name_should_return_ok() {
  let ms = MockServer::start();

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(DELETE)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists:by_name")
      .query_param("name", "test-name");

    then.status(200).header("Content-Type", "application/json");
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client
    .delete_watch_list_by_name("test-name".to_string())
    .await
  {
    Ok(_) => {
      endpoint_mock.assert();
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_delete_asset_from_watch_list_should_return_ok() {
  let ms = MockServer::start();
  let response_body = r#"
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "account_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "created_at": "2025-12-02T10:29:22.079Z",
    "updated_at": "2025-12-02T10:29:22.079Z",
    "name": "string",
    "assets": [
      {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "class": "us_equity",
        "cusip": "987654321",
        "exchange": "NYSE",
        "symbol": "AAPL",
        "name": "string",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true,
        "margin_requirement_long": "1234",
        "margin_requirement_short": "523",
        "attributes": [
          "ptp_no_exception",
          "ipo"
        ]
      }
    ]
  }
  "#;

  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(DELETE)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/watchlists/3fa85f64-5717-4562-b3fc-2c963f66afa6/META");

    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  match api_client
    .delete_asset_from_watch_list(
      Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap(),
      "META".to_string(),
    )
    .await
  {
    Ok(watchlist) => {
      assert_eq!(
        watchlist.id,
        Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap()
      )
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}
