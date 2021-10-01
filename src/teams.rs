use std::ops::Deref;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::requests::get;

#[derive(Serialize, Deserialize)]
pub struct Teams {
    #[serde(rename = "copyright")]
    copyright: String,

    #[serde(rename = "teams")]
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "league")]
    pub league: League,

    #[serde(rename = "teamName")]
    pub team_name: String,

    #[serde(rename = "shortName")]
    pub short_name: String,

    #[serde(rename = "abbreviation")]
    pub abbreviation: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct League {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "link")]
    link: String,

    #[serde(rename = "abbreviation")]
    abbreviation: Option<String>,
}

impl Teams {
    pub async fn get_teams() -> Result<Teams> {
        get("teams").await
    }
}

impl Deref for Teams {
    type Target = Vec<Team>;

    fn deref(&self) -> &Self::Target {
        &self.teams
    }
}
