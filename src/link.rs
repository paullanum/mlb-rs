use std::marker::PhantomData;

use crate::requests::Request;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Link<T: for<'d> Deserialize<'d>> {
    _phantom: PhantomData<T>,
    link: String,
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
