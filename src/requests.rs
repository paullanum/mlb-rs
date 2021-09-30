use async_trait::async_trait;
use std::borrow::Borrow;

use anyhow::Result;
use reqwest::Url;
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://statsapi.mlb.com";
const DEFAULT_API: &str = "api/v1/";

pub async fn get<T>(address: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    _get(create_url(address)?).await
}

fn create_url(param: &str) -> Result<Url> {
    Ok(Url::parse(&format!(
        "{}/{}{}",
        BASE_URL, DEFAULT_API, param
    ))?)
}

async fn _get<T>(address: Url) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(reqwest::get(address).await?.json().await?)
}

pub async fn get_with_params<'a, T, I>(root: &str, params: I) -> Result<T>
where
    T: DeserializeOwned,
    I: IntoIterator,
    I::Item: Borrow<(&'a str, &'a str)>,
{
    _get(Url::parse_with_params(create_url(root)?.as_str(), params)?).await
}

#[async_trait]
pub trait GetLink {
    fn get_link(&self) -> &str;
    async fn goto_link<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        log::debug!("{}{}", BASE_URL, self.get_link());
        _get(Url::parse(&format!("{}{}", BASE_URL, self.get_link()))?).await
    }
}
