use std::str::FromStr;

use chrono::Duration;
use serde::Serialize;

use anyhow::Result;
use clap::command;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use pomobar_rs::{Pomobar, State};

#[derive(Debug, Clone, Serialize)]
struct Waybar {
    text: String,
    class: String,
}

impl From<Pomobar> for Waybar {
    fn from(value: Pomobar) -> Self {
        let minutes = value.remaining_time.num_minutes();
        let seconds = value
            .remaining_time
            .checked_sub(&Duration::minutes(minutes))
            .unwrap()
            .num_seconds();

        let time = format!("{:02}:{:02}", minutes, seconds);

        let text = match value.state {
            State::Idle => format!(" {}", &time),
            State::Paused => format!("󰏤 {}", &time),
            State::Work | State::ShortBreak | State::LongBreak => format!(" {}", &time),
        };

        let class = value.state.to_string();

        Self { text, class }
    }
}

impl std::fmt::Display for Waybar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = serde_json::to_string(&self).unwrap();
        f.write_str(&content)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = "/tmp/pomobar.sock";

    let cmd = command!()
        .subcommand(command!("status"))
        .subcommand(command!("toggle"))
        .subcommand(command!("reset"));

    let matches = cmd.get_matches();

    let mut socket = UnixStream::connect(path).await?;

    let command = matches.subcommand_name().unwrap();

    socket.write_all(command.as_bytes()).await?;

    let mut buf = vec![0; 1024];

    let n = socket.read(&mut buf).await.unwrap();

    let content = String::from_utf8(buf[..n].to_vec()).unwrap();

    println!("{}", Waybar::from(Pomobar::from_str(&content).unwrap()));

    Ok(())
}
