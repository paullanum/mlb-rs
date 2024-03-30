use std::marker::PhantomData;

use crate::requests::Request;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct Link<T> {
    _phantom: PhantomData<T>,
    link: String,
}

impl<'de, T> Deserialize<'de> for Link<T> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let value = Deserialize::deserialize(deserializer)?;
        Ok(Self {
            _phantom: PhantomData,
            link: value,
        })
    }
}

impl<T: for<'de> Deserialize<'de>> Link<T> {
    pub async fn follow(&self) -> Result<T> {
        Request::new()
            .with_api("")
            .with_endpoint(self.link.as_str())
            .get()
            .await
    }
}
