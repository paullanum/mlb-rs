extern crate serde_json;
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use tabled::Tabled;

#[derive(Deserialize, Clone, Debug)]
pub struct LiveGame {
    #[serde(rename = "gamePk")]
    game_pk: i64,

    #[serde(rename = "link")]
    link: String,

    #[serde(rename = "gameData")]
    game_data: GameData,

    #[serde(rename = "liveData")]
    live_data: LiveData,
}

#[derive(Deserialize, Clone, Debug)]
struct GameData {
    #[serde(rename = "teams")]
    teams: GameDataTeams,

    #[serde(rename = "datetime")]
    date_time: GameDateTime,
}

#[derive(Deserialize, Clone, Debug)]
struct GameDateTime {
    #[serde(rename = "dateTime", with = "date_format")]
    date_time: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Debug)]
struct GameDataTeams {
    #[serde(rename = "away")]
    away: GameDataTeam,

    #[serde(rename = "home")]
    home: GameDataTeam,
}

#[derive(Deserialize, Clone, Debug)]
struct GameDataTeam {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "abbreviation")]
    abbreviation: String,

    #[serde(rename = "teamName")]
    team_name: String,

    #[serde(rename = "record")]
    record: Record,
}

#[derive(Deserialize, Clone, Debug)]
struct Record {
    #[serde(rename = "wins")]
    wins: i64,

    #[serde(rename = "losses")]
    losses: i64,
}

#[derive(Deserialize, Clone, Debug)]
struct LiveData {
    #[serde(rename = "linescore")]
    linescore: Linescore,
}

// TODO: Add in other details here
#[derive(Deserialize, Clone, Debug)]
struct Linescore {
    #[serde(rename = "currentInning")]
    current_inning: Option<i64>,

    #[serde(rename = "inningState")]
    inning_state: Option<String>,

    #[serde(rename = "teams")]
    teams: LinescoreTeams,
}

#[derive(Deserialize, Clone, Debug)]
struct TeamLinescore {
    #[serde(rename = "runs")]
    runs: i64,

    #[serde(rename = "hits")]
    hits: i64,

    #[serde(rename = "errors")]
    errors: i64,

    #[serde(rename = "leftOnBase")]
    left_on_base: i64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum OrEmpty<T> {
    Full(T),
    Empty {},
}

#[derive(Deserialize, Clone, Debug)]
struct LinescoreTeams {
    #[serde(rename = "home")]
    home: OrEmpty<TeamLinescore>,

    #[serde(rename = "away")]
    away: OrEmpty<TeamLinescore>,
}

#[derive(Debug)]
pub struct BoxScore {
    pub away: TeamBoxScore,
    pub home: TeamBoxScore,
    pub inning_state: String,
    pub inning: i64,
}

#[derive(Tabled, Debug)]
pub struct TeamBoxScore {
    #[tabled(rename = "Team")]
    team: String,

    #[tabled(rename = "R")]
    runs: i64,

    #[tabled(rename = "H")]
    hits: i64,

    #[tabled(rename = "E")]
    errors: i64,
}

#[derive(Debug)]
pub struct GameStart<'a> {
    teams: &'a GameDataTeams,
    start_time: DateTime<Utc>,
}

mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

impl TeamLinescore {
    fn make_boxscore(&self, team: &str) -> TeamBoxScore {
        TeamBoxScore {
            team: team.to_string(),
            runs: self.runs,
            hits: self.hits,
            errors: self.errors,
        }
    }
}

impl Linescore {
    fn make_boxscore(&self, away_name: &str, home_name: &str) -> Option<BoxScore> {
        match (self.teams.away.clone(), self.teams.home.clone()) {
            (OrEmpty::Full(away), OrEmpty::Full(home)) => Some(BoxScore {
                away: away.make_boxscore(away_name),
                home: home.make_boxscore(home_name),
                inning_state: self.inning_state.clone()?,
                inning: self.current_inning?,
            }),
            _ => None,
        }
    }
}

impl LiveGame {
    pub fn get_score(&self) -> Option<BoxScore> {
        let teams = &self.game_data.teams;
        self.live_data
            .linescore
            .make_boxscore(&teams.away.abbreviation, &teams.home.abbreviation)
    }
    pub fn current_inning(&self) -> Option<String> {
        let linescore = self.live_data.linescore.clone();
        if let (Some(inning_state), Some(inning)) =
            (linescore.inning_state, linescore.current_inning)
        {
            Some(inning_state + " " + &inning.to_string())
        } else {
            None
        }
    }
    pub fn get_start_time(&self) -> Option<GameStart> {
        Some(GameStart {
            teams: &self.game_data.teams,
            start_time: self.game_data.date_time.date_time,
        })
    }
}

impl std::fmt::Display for GameStart<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} @ {} - {}",
            self.teams.away.abbreviation,
            self.teams.home.abbreviation,
            self.start_time.with_timezone(&Local).format("%I:%M %p")
        )
    }
}
