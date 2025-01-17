use std::{
    path::Path,
    sync::{LazyLock, Mutex},
};

use anyhow::Result;
use chrono::Duration;
use serde::Serialize;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixListener,
};

use pomobar_rs::{Pomobar, State};

static POMOBAR: LazyLock<Mutex<Pomobar>> = LazyLock::new(|| Mutex::new(Pomobar::default()));

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

    if Path::new(path).exists() {
        std::fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn({
            async move {
                let reader = BufReader::new(stream);
                let mut lines = reader.lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    let mut pomobar = POMOBAR.lock().unwrap().clone();

                    let pomobar = match line.as_str() {
                        "reset" => pomobar.reset(),
                        "toggle" => pomobar.toggle(),
                        _ => pomobar.status(),
                    };

                    *POMOBAR.lock().unwrap() = pomobar.clone();

                    let waybar = Waybar::from(pomobar);

                    println!("{}", waybar);
                }
            }
        });
    }
}
