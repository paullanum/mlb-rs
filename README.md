# `MLB-rs`

## Access the MLB stats API as a library and a CLI tool

----

> :warning: **Warning:** This project is still in very early development; it is not representative of what a final interface may look like, and may not even work.
> Use this project at your own risk, and certainly don't depend on it.

----

> :balance_scale: **Licence Info:** Neither this project nor I am associated with MLB in any way.
> Only the code contained within the project is available under the license specified in the `LICENSE` file.
> This project queries the MLB API, whose usage agreement is available [here](http://gdx.mlb.com/components/copyright.txt).
> Though this project contains a library to query the API, it does not imply any license of ways in which said API or its data may be used.
> All data retrieved is subject to MLB copyright and must follow their usage agreement

## State of the Project

This project is still in a very early state. While it is functional, it is neither feature complete nor API-stable.

## CLI Commands

| Command  | State              | Arguments | Description                                                                                                                           |
| -------- | ------------------ | --------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `scores` | :white_check_mark: | `Team`    | Get score for a selected team or *all teams (default)*. If a game is in progress or completed, it is displayed in a linescore format. |
| `teams`  | :white_check_mark: |           | Show a list of teams which can be used to search for active games.                                                                    |

## CLI Configuration

Configuration can be done through command line arguments or a configuration file. The configuration file is loaded from the path set in the `MLB_SETTINGS` environment variable, or `HOME\mlb_settings.toml` by default.

### Available Settings

| Setting | Command Line Argument  | TOML Setting    |
| ------- | ---------------------- | --------------- |
| `Team`  | `-t`/`--team` `{name}` | `team` (string) |

## Current Todo List

- [ ] Add alternative version(s) of commands (i.e. a small, one-line version of scores).
- [ ] Create a `config` subcommand to generate a file and display options.
- [ ] Add a scoreboard subcommand (likely a variant of scores) which displays all currently active games, even when a team is chosen in the config file.
- [ ] Only show each game once (currently shows each team's games, so a NYY vs BOS game would show up once for NYY and once for BOS)
- [ ] Add more documentation on using this project as a library as well as a command line tool.
- [ ] Update and add doc comments, tests, and overall documentation.

## Longer-term Todo

- [ ] Cache requests which can be reused multiple times (i.e. only request a team's schedule once per day/week/season/etc).
- [ ] Ability to search for historic game information.
