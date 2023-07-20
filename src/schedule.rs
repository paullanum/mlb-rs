use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{link::Link, live::LiveGame, requests::Request};

#[derive(Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "copyright")]
    pub copyright: String,

    #[serde(rename = "totalItems")]
    pub total_items: i64,

    #[serde(rename = "totalEvents")]
    pub total_events: i64,

    #[serde(rename = "totalGames")]
    pub total_games: i64,

    #[serde(rename = "totalGamesInProgress")]
    pub total_games_in_progress: i64,

    #[serde(rename = "dates")]
    pub dates: Vec<Date>,
}

#[derive(Serialize, Deserialize)]
pub struct Date {
    #[serde(rename = "date")]
    pub date: String,

    #[serde(rename = "totalItems")]
    pub total_items: i64,

    #[serde(rename = "totalEvents")]
    pub total_events: i64,

    #[serde(rename = "totalGames")]
    pub total_games: i64,

    #[serde(rename = "totalGamesInProgress")]
    pub total_games_in_progress: i64,

    #[serde(rename = "games")]
    pub games: Vec<Game>,

    #[serde(rename = "events")]
    pub events: Vec<Option<Event>>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    #[serde(rename = "gamePk")]
    pub game_pk: i64,

    #[serde(rename = "link")]
    pub link: Link<LiveGame>,

    #[serde(rename = "teams")]
    pub teams: Teams,

    #[serde(rename = "publicFacing")]
    pub public_facing: bool,

    #[serde(rename = "doubleHeader")]
    pub double_header: String,

    #[serde(rename = "scheduledInnings")]
    pub scheduled_innings: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Status {
    #[serde(rename = "abstractGameState")]
    pub abstract_game_state: String,

    #[serde(rename = "codedGameState")]
    pub coded_game_state: String,

    #[serde(rename = "detailedState")]
    pub detailed_state: String,

    #[serde(rename = "statusCode")]
    pub status_code: String,

    #[serde(rename = "startTimeTBD")]
    pub start_time_tbd: bool,

    #[serde(rename = "abstractGameCode")]
    pub abstract_game_code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Teams {
    #[serde(rename = "away")]
    pub away: Team,

    #[serde(rename = "home")]
    pub home: Team,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    #[serde(rename = "leagueRecord")]
    pub league_record: LeagueRecord,

    #[serde(rename = "team")]
    pub team: Venue,

    #[serde(rename = "splitSquad")]
    pub split_squad: bool,

    #[serde(rename = "seriesNumber")]
    pub series_number: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeagueRecord {
    #[serde(rename = "wins")]
    pub wins: i64,

    #[serde(rename = "losses")]
    pub losses: i64,

    #[serde(rename = "pct")]
    pub pct: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Venue {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,
}

pub async fn get_schedule<T: std::string::ToString>(team_id: T) -> Result<Schedule> {
    Request::new()
        .with_endpoint("schedule")
        .with_params([("sportId", "1"), ("teamId", &team_id.to_string())])
        .get()
        .await
}
