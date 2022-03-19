use anyhow::Result;
use reqwest::Url;
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://statsapi.mlb.com";
const DEFAULT_API: &str = "api/v1/";

pub struct Request<'a> {
    base_url: &'a str,
    api: &'a str,
    endpoint: &'a str,
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            base_url: BASE_URL,
            api: DEFAULT_API,
            endpoint: Default::default(),
            params: Default::default(),
        }
    }

    pub fn with_endpoint(self, endpoint: &'a str) -> Self {
        Self { endpoint, ..self }
    }

    pub fn with_params<I: IntoIterator<Item = (&'a str, &'a str)>>(self, params: I) -> Self {
        Self {
            params: params.into_iter().collect(),
            ..self
        }
    }

    pub fn with_api(self, api: &'a str) -> Self {
        Self { api, ..self }
    }

    pub async fn get<T: DeserializeOwned>(self) -> Result<T> {
        let base = format!("{}/{}{}", self.base_url, self.api, self.endpoint);
        let address = match self.params.len() {
            0 => Url::parse(&base)?,
            _ => Url::parse_with_params(&base, self.params)?,
        };
        Ok(reqwest::get(address).await?.json().await?)
    }
}

impl<'a> Default for Request<'a> {
    fn default() -> Self {
        Self::new()
    }
}
