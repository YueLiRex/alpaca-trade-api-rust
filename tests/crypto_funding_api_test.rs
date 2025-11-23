use alpaca_trade_api_rust::{
  api::{
    CryptoFundingApi,
    CryptonNetwork,
    FundingWalletsParameter,
  },
  prelude::Client,
};
use httpmock::{
  Method::GET,
  MockServer,
};

mod shared;

#[tokio::test]
async fn test_get_all_crypto_funding_wallet_should_return_good() {
  let mock_server = MockServer::start();
  let base_url = mock_server.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  let test_context = crate::shared::TestContext::new(&mock_server, &api_client);
  let response_body = r#"[
            {
              "chain": "ethereum",
              "address": "adsfafewf-fwefqwfqwfweqf",
              "created_at": "2025-11-15T22:34:58+01:00"
            },
            {
              "chain": "solana",
              "address": "ewrqfqfqfqwefqwfe",
              "created_at": "2025-11-15T22:34:58+01:00"
            }
          ]"#;

  let parameter = FundingWalletsParameter {
    asset: Some("bitcoin".into()),
    network: Some(CryptonNetwork::Ethereum),
  };

  test_context
    .setup_endpoint(
      GET,
      "/v2/wallets",
      200,
      response_body,
      |client| async move {
        match client.get_all_crypto_funding_wallet(&parameter).await {
          Ok(wallet_info_list) => {
            assert_eq!(wallet_info_list.len(), 2)
          }
          Err(error) => panic!("API call failed: {:?}", error),
        }
      },
    )
    .await;
}

#[tokio::test]
async fn test_get_all_crypto_funding_transfer_should_return_good() {
  let mock_server = MockServer::start();
  let base_url = mock_server.base_url();
  let api_client = Client::new(base_url, "test_key".to_string(), "test_secret".to_string());

  let test_context = crate::shared::TestContext::new(&mock_server, &api_client);
  let response_body = r#"[
      {
        "id": "ee205f1c-79a8-403f-825b-438b56051076",
        "tx_hash": "0xabc...xyz",
        "direction": "INCOMING",
        "status": "PROCESSING",
        "amount": "100",
        "usd_value": "133",
        "network_fee": "10",
        "fees": "3.8",
        "chain": "bitcoin",
        "asset": "BTC",
        "from_address": "safasfwef",
        "to_address": "sfsadsdvwev",
        "created_at": "2025-11-15T22:34:58+01:00"
      },
      {
        "id": "ee205f1c-79a8-403f-825b-438b56051076",
        "tx_hash": "0xabc...xyz",
        "direction": "INCOMING",
        "status": "PROCESSING",
        "amount": "100",
        "usd_value": "133",
        "network_fee": "10",
        "fees": "3.8",
        "chain": "bitcoin",
        "asset": "BTC",
        "from_address": "safasfwef",
        "to_address": "sfsadsdvwev",
        "created_at": "2025-11-15T22:34:58+01:00"
      }
    ]"#;

  test_context
    .setup_endpoint(
      GET,
      "/v2/wallets/transfers",
      200,
      response_body,
      |client| async move {
        match client.get_all_crypto_funding_transfer().await {
          Ok(transfer_info_list) => {
            assert_eq!(transfer_info_list.len(), 2)
          }
          Err(error) => panic!("API call failed: {:?}", error),
        }
      },
    )
    .await;
}

#[tokio::test]
async fn test_new_withdrawal_should_return_ok() {}
