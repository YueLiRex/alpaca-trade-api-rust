use crate::{
  client::Client,
  models::{
    ErrorResponse,
    WatchList,
  },
};
use anyhow::bail;
use chrono::{
  DateTime,
  Utc,
};
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

pub trait WatchListApi {
  fn get_all_watch_lists(&self) -> impl Future<Output = anyhow::Result<Vec<BasicWatchListInfo>>>;

  fn create_watch_list(&self, request_body: WatchListReqBody) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn get_watch_list_by_id(&self, watchlist_id: Uuid) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn update_watch_list_by_id(
    &self,
    watchlist_id: Uuid,
    request_body: WatchListReqBody,
  ) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn add_asset_to_watch_list(
    &self,
    watchlist_id: Uuid,
    symbol: AddAssetReqBody,
  ) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn delete_watch_list_by_id(&self, watchlist_id: Uuid) -> impl Future<Output = anyhow::Result<()>>;

  fn get_watch_list_by_name(&self, watchlist_name: String) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn update_watch_list_by_name(
    &self,
    watchlist_name: String,
    request_body: WatchListReqBody,
  ) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn add_asset_to_watch_list_by_name(
    &self,
    watchlist_name: String,
    symbol: AddAssetReqBody,
  ) -> impl Future<Output = anyhow::Result<WatchList>>;

  fn delete_watch_list_by_name(&self, name: String) -> impl Future<Output = anyhow::Result<()>>;

  fn delete_asset_from_watch_list(
    &self,
    watchlist_id: Uuid,
    symbol: String,
  ) -> impl Future<Output = anyhow::Result<WatchList>>;
}

impl WatchListApi for Client {
  async fn get_all_watch_lists(&self) -> anyhow::Result<Vec<BasicWatchListInfo>> {
    let url = format!("{}/v2/watchlists", self.base_url);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_lists = response.json::<Vec<BasicWatchListInfo>>().await?;
          Ok(watch_lists)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn create_watch_list(&self, request_body: WatchListReqBody) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists", self.base_url);

    match self.client.post(url).json(&request_body).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn get_watch_list_by_id(&self, watchlist_id: Uuid) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists/{}", self.base_url, watchlist_id);
    match self.client.get(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn update_watch_list_by_id(
    &self,
    watchlist_id: Uuid,
    request_body: WatchListReqBody,
  ) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists/{}", self.base_url, watchlist_id);
    match self.client.put(url).json(&request_body).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn add_asset_to_watch_list(&self, watchlist_id: Uuid, symbol: AddAssetReqBody) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists/{}", self.base_url, watchlist_id);
    match self.client.post(url).json(&symbol).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn delete_watch_list_by_id(&self, watchlist_id: Uuid) -> anyhow::Result<()> {
    let url = format!("{}/v2/watchlists/{}", self.base_url, watchlist_id);
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

  async fn get_watch_list_by_name(&self, watchlist_name: String) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists:by_name", self.base_url);
    match self.client.get(url).query(&vec![("name", watchlist_name)]).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn update_watch_list_by_name(
    &self,
    watchlist_name: String,
    request_body: WatchListReqBody,
  ) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists:by_name", self.base_url);
    match self
      .client
      .put(url)
      .query(&vec![("name", watchlist_name)])
      .json(&request_body)
      .send()
      .await
    {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn add_asset_to_watch_list_by_name(
    &self,
    watchlist_name: String,
    symbol: AddAssetReqBody,
  ) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists:by_name", self.base_url);
    match self
      .client
      .post(url)
      .query(&vec![("name", watchlist_name)])
      .json(&symbol)
      .send()
      .await
    {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }

  async fn delete_watch_list_by_name(&self, name: String) -> anyhow::Result<()> {
    let url = format!("{}/v2/watchlists:by_name", self.base_url);
    match self.client.delete(url).query(&vec![("name", name)]).send().await {
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

  async fn delete_asset_from_watch_list(&self, watchlist_id: Uuid, symbol: String) -> anyhow::Result<WatchList> {
    let url = format!("{}/v2/watchlists/{}/{}", self.base_url, watchlist_id, symbol);
    match self.client.delete(url).send().await {
      Ok(response) => {
        if response.status().is_success() {
          let watch_list = response.json::<WatchList>().await?;
          Ok(watch_list)
        } else {
          let error_response = response.json::<ErrorResponse>().await?;
          bail!(error_response)
        }
      }
      Err(error) => bail!(error),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct BasicWatchListInfo {
  pub id: Uuid,
  pub account_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct WatchListReqBody {
  pub name: String,
  pub symbols: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AddAssetReqBody {
  pub symbol: String,
}

#[cfg(test)]
mod tests {
  use crate::api::WatchListReqBody;

  #[test]
  fn test_watch_list_req_body() {
    let request_body = WatchListReqBody {
      name: "test".to_string(),
      symbols: vec!["META".to_string(), "GOGL".to_string()],
    };

    let serialized = serde_json::to_string(&request_body).unwrap();
    let expected = r#"{"name":"test","symbols":["META","GOGL"]}"#;
    assert_eq!(serialized, expected)
  }
}
