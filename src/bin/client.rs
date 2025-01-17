#[macro_use]
extern crate tracing;

use anyhow::Result;
use clap::command;
use tokio::{io::AsyncWriteExt, net::UnixStream};
use tracing_subscriber::{prelude::*, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    let stdout = tracing_subscriber::fmt::Layer::new().pretty();

    Registry::default().with(stdout).init();

    let path = "/tmp/pomobar.sock";

    info!("連線 socket 成功");

    let mut stream = UnixStream::connect(path).await?;

    let cmd = command!()
        .subcommand(command!("status"))
        .subcommand(command!("toggle"))
        .subcommand(command!("reset"));

    let matches = cmd.get_matches();

    stream
        .write_all(matches.subcommand_name().unwrap_or("status").as_bytes())
        .await?;

    Ok(())
}
