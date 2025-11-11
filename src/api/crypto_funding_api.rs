use crate::{
  client::Client,
  models::{
    CryptoTransfer,
    CryptoWalletInfo,
    ErrorResponse,
    GasFee,
    WhiteListedAddress,
    utils::Money,
  },
};
use anyhow::bail;
use serde::Serialize;

pub trait CryptoFundingApi {
  fn get_all_crypto_funding_wallet(
    &self,
    request_parameter: FundingWalletsParameter,
  ) -> impl Future<Output = anyhow::Result<Vec<CryptoWalletInfo>>>;

  fn get_all_crypto_funding_transfer(&self)
  -> impl Future<Output = anyhow::Result<CryptoTransfer>>;

  fn new_withdrawal(
    &self,
    request_body: WithdrawalReqBody,
  ) -> impl Future<Output = anyhow::Result<CryptoTransfer>>;

  fn get_crypto_funding_transfer(
    &self,
    transfer_id: String,
  ) -> impl Future<Output = anyhow::Result<CryptoTransfer>>;

  fn get_whitelisted_addresses(&self) -> impl Future<Output = anyhow::Result<WhiteListedAddress>>;

  fn new_whitelisted_address(
    &self,
    request_body: WhitelistedAddressReqBody,
  ) -> impl Future<Output = anyhow::Result<WhiteListedAddress>>;

  fn delete_whitelisted_address(
    &self,
    whitelisted_address_id: String,
  ) -> impl Future<Output = anyhow::Result<()>>;

  fn return_estimate_gas_fee(
    &self,
    request_parameter: ReturnGasFeeParameter,
  ) -> impl Future<Output = anyhow::Result<GasFee>>;

}

impl CryptoFundingApi for Client {
  async fn get_all_crypto_funding_wallet(
    &self,
    request_parameter: FundingWalletsParameter,
  ) -> anyhow::Result<Vec<CryptoWalletInfo>> {
    let url = format!("{}/v2/wallets", self.base_url);
    match self.client.get(url).query(&request_parameter).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let wallets = response.json::<Vec<CryptoWalletInfo>>().await?;
          Ok(wallets)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_all_crypto_funding_transfer(&self) -> anyhow::Result<CryptoTransfer> {
    let url = format!("{}/v2/wallets/transfers", self.base_url);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let transfers = response.json::<CryptoTransfer>().await?;
          Ok(transfers)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn new_withdrawal(
    &self,
    request_body: WithdrawalReqBody,
  ) -> anyhow::Result<CryptoTransfer> {
    let url = format!("{}/v2/wallets", self.base_url);
    match self.client.post(url).json(&request_body).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let transfer = response.json::<CryptoTransfer>().await?;
          Ok(transfer)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_crypto_funding_transfer(
    &self,
    transfer_id: String,
  ) -> anyhow::Result<CryptoTransfer> {
    let url = format!("{}/v2/wallets/transfers/{}", self.base_url, transfer_id);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let transfer = response.json::<CryptoTransfer>().await?;
          Ok(transfer)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_whitelisted_addresses(&self) -> anyhow::Result<WhiteListedAddress> {
    let url = format!("{}/v2/wallets/whitelists", self.base_url);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let addresses = response.json::<WhiteListedAddress>().await?;
          Ok(addresses)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn new_whitelisted_address(
    &self,
    request_body: WhitelistedAddressReqBody,
  ) -> anyhow::Result<WhiteListedAddress> {
    let url = format!("{}/v2/wallets/whitelists", self.base_url);
    match self.client.post(url).json(&request_body).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let addresses = response.json::<WhiteListedAddress>().await?;
          Ok(addresses)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn delete_whitelisted_address(&self, whitelisted_address_id: String) -> anyhow::Result<()> {
    let url = format!("{}/v2/wallets/whitelists/{}", self.base_url, whitelisted_address_id);
    match self.client.delete(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          Ok(())
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn return_estimate_gas_fee(
    &self,
    request_parameter: ReturnGasFeeParameter,
  ) -> anyhow::Result<GasFee> {
    let url = format!("{}/v2/wallets/fees/estimate", self.base_url);
    match self.client.get(url).query(&request_parameter).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let gasfee = response.json::<GasFee>().await?;
          Ok(gasfee)
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
pub struct FundingWalletsParameter {
  pub asset: Option<String>,
  pub network: Option<CryptonNetwork>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CryptonNetwork {
  Ethereum,
  Solana,
}

#[derive(Debug, Serialize)]
pub struct WithdrawalReqBody {
  pub amount: Money,
  pub address: String,
  pub asset: String,
}

#[derive(Debug, Serialize)]
pub struct WhitelistedAddressReqBody {
  pub address: String,
  pub asset: String,
}

#[derive(Debug, Serialize)]
pub struct ReturnGasFeeParameter {
  pub asset: Option<String>,
  pub from_address: Option<String>,
  pub to_address: Option<String>,
  pub amount: Option<Money>,
}
