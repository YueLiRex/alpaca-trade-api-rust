use alpaca_trade_api_rust::{
  api::{
    ComaSeparatedStrings,
    DefaultBoolean,
    OptionApi,
    OptionContractsQueryParameter,
  },
  prelude::{
    Client,
    OptionStatus,
    OptionStyle,
    OptionType,
  },
};
use httpmock::{
  Method::GET,
  MockServer,
};

#[tokio::test]
async fn test_get_option_contracts_should_return_all() {
  let ms = MockServer::start();
  let mock_response_body = r#"
  {
    "option_contracts": [
      {
        "id": "0c7826be-8606-4100-9e0a-94a1b6f5aaad",
        "symbol": "AA251114C00020000",
        "name": "AA Nov 14 2025 20 Call",
        "status": "active",
        "tradable": true,
        "expiration_date": "2025-11-14",
        "root_symbol": "AA",
        "underlying_symbol": "AA",
        "underlying_asset_id": "3ca0202f-01f4-41a0-bb0c-c8864e767ebd",
        "type": "call",
        "style": "american",
        "strike_price": "20",
        "multiplier": "100",
        "size": "100",
        "open_interest": null,
        "open_interest_date": null,
        "close_price": null,
        "close_price_date": null,
        "ppind": true
      },
      {
        "id": "f8df3699-b0a4-4666-9bd9-ebf129dcdab3",
        "symbol": "AA251114C00024000",
        "name": "AA Nov 14 2025 24 Call",
        "status": "active",
        "tradable": true,
        "expiration_date": "2025-11-14",
        "root_symbol": "AA",
        "underlying_symbol": "AA",
        "underlying_asset_id": "3ca0202f-01f4-41a0-bb0c-c8864e767ebd",
        "type": "call",
        "style": "american",
        "strike_price": "24",
        "multiplier": "100",
        "size": "100",
        "open_interest": null,
        "open_interest_date": null,
        "close_price": "13.65",
        "close_price_date": "2025-11-07",
        "ppind": true
      }
    ],
    "next_page_token": "MTAw"
  }
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/options/contracts");
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });
  let parameter = &OptionContractsQueryParameter {
    underlying_symbols: Some(ComaSeparatedStrings {
      values: vec!["appl", "tsla"],
    }),
    status: OptionStatus::Active,
    show_deliverables: DefaultBoolean { value: true },
    expiration_date: None,
    expiration_date_gte: Some("2025-01-23".to_string()),
    expiration_date_lte: None,
    root_symbol: Some("AAPL".to_string()),
    _type: Some(OptionType::Call),
    style: Some(OptionStyle::American),
    strike_price_gte: Some(23.32),
    strike_price_lte: None,
    page_token: Some("test-token".to_string()),
    limit: Some(100),
    ppind: None,
  };

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  match api_client.get_option_contracts(parameter).await {
    Ok(response) => {
      let option_list = response.option_contracts;
      let next_page_token = response.next_page_token;

      assert_eq!(option_list.len(), 2);
      assert_eq!(next_page_token, Some("MTAw".to_string()));
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}

#[tokio::test]
async fn test_get_specific_option_contracts_should_return_one() {
  let ms = MockServer::start();
  let mock_response_body = r#"
  {
    "id": "f8df3699-b0a4-4666-9bd9-ebf129dcdab3",
    "symbol": "AA251114C00024000",
    "name": "AA Nov 14 2025 24 Call",
    "status": "active",
    "tradable": true,
    "expiration_date": "2025-11-14",
    "root_symbol": "AA",
    "underlying_symbol": "AA",
    "underlying_asset_id": "3ca0202f-01f4-41a0-bb0c-c8864e767ebd",
    "type": "call",
    "style": "american",
    "strike_price": "24",
    "multiplier": "100",
    "size": "100",
    "open_interest": null,
    "open_interest_date": null,
    "close_price": "13.65",
    "close_price_date": "2025-11-07",
    "deliverables": [
      {
        "type": "equity",
        "symbol": "AA",
        "asset_id": "3ca0202f-01f4-41a0-bb0c-c8864e767ebd",
        "amount": "100",
        "allocation_percentage": "100",
        "settlement_type": "T+1",
        "settlement_method": "CCC",
        "delayed_settlement": false
      }
    ],
    "ppind": true
  }
  "#;
  let endpoint_mock = ms.mock(|when, then| {
    when
      .method(GET)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", "test_key")
      .header("APCA-API-SECRET-KEY", "test_secret")
      .path("/v2/options/contracts/AA251114C00024000");
    then
      .status(200)
      .header("Content-Type", "application/json")
      .body(mock_response_body);
  });

  let base_url = ms.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());
  match api_client
    .get_option_contract_by_symbol_or_id("AA251114C00024000")
    .await
  {
    Ok(option_contract) => {
      assert_eq!(option_contract.deliverables.map(|d| d.len()), Some(1));
      assert_eq!(option_contract.size.value(), 100)
    }
    Err(error) => {
      endpoint_mock.assert();
      panic!("Error: {}", error)
    }
  }
}
