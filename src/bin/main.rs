use std::env;

use anyhow::Result;
use clap::{App, Arg};
use config::{Config, ConfigError, File};
use mlb::{live::LiveGame, requests::GetLink};
use serde::Deserialize;
use serde_json::from_str;
use tabled::{Style, Table};
use tokio_stream::StreamExt;

const AVAILABLE_LEAGUES: &[&str] = &["american league", "national league"];
const TEAMS: &str = include_str!(concat!(env!("OUT_DIR"), "/teams.json"));

fn is_in_league(team: &mlb::teams::Team) -> bool {
    AVAILABLE_LEAGUES.contains(
        &team
            .league
            .name
            .as_deref()
            .unwrap_or("ERROR")
            .to_lowercase()
            .as_str(),
    )
}

// TODO: Make this return a vector of games to properly handle doubleheaders.
async fn get_game(team: mlb::teams::Team) -> Option<LiveGame> {
    let schedule = mlb::schedule::get_schedule(team.id).await.ok()?;
    if !schedule.dates.is_empty() {
        schedule
            .dates
            .first()?
            .games
            .first()?
            .goto_link::<LiveGame>()
            .await
            .ok()
    } else {
        None
    }
}

async fn scores(team_name: Option<&str>) -> Result<()> {
    let teams: mlb::teams::Teams = from_str(TEAMS)?;
    let filter: Box<dyn Fn(&mlb::teams::Team) -> bool> = match team_name {
        Some(_) => Box::new(|team| {
            team.team_name.to_lowercase().contains(team_name.unwrap()) && is_in_league(team)
        }),
        None => Box::new(|team| is_in_league(team)),
    };
    let mut all_scores = tokio_stream::iter(teams.teams).filter_map(|team| {
        if filter(&team) {
            Some(get_game(team))
        } else {
            None
        }
    });
    while let Some(g) = all_scores.next().await {
        if let Some(game) = g.await {
            if let Some(score) = game.get_score() {
                println!(
                    "{}{} {}",
                    Table::new([score.away, score.home]).with(Style::pseudo()),
                    score.inning_state,
                    score.inning
                );
            } else {
                println!(
                    "{}",
                    game.get_start_time().expect("No start time available")
                );
            }
        } else {
            println!("No game(s) today")
        }
    }
    Ok(())
}

async fn teams() -> Result<()> {
    let teams: mlb::teams::Teams = from_str(TEAMS)?;
    let mut all_teams = tokio_stream::iter(teams.teams).filter(is_in_league);
    while let Some(team) = all_teams.next().await {
        println!("{}", team.team_name);
    }
    Ok(())
}

#[derive(Default, Deserialize)]
struct Configuration {
    team: Option<String>,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        if let Ok(name) = env::var("MLB_CONFIG") {
            s.merge(File::with_name(&name))?;
        } else {
            let file_path = dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from(""))
                .join("mlb_settings");
            s.merge(File::with_name(file_path.to_str().unwrap_or("")).required(false))?;
        }

        s.try_into()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let settings = Configuration::new()?;
    let app = App::new("mlb")
        .subcommand(
            App::new("scores")
                .about("Display scores for live games")
                .arg(
                    Arg::with_name("team")
                        .short("t")
                        .takes_value(true)
                        .help("Specify a team to search for"),
                ),
        )
        .subcommand(App::new("teams").about("Display available teams"));
    let matches = app.get_matches();
    match matches.subcommand() {
        ("scores", Some(name)) => {
            scores(name.value_of("team").or_else(|| settings.team.as_deref())).await?
        }
        ("teams", _) => teams().await?,
        ("config", Some(_)) => todo!("Add config function"),
        _ => log::error!("Error!"),
    }
    Ok(())
}
