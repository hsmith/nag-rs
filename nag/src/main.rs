//
// 2024
// SPDX-License-Identifier: MIT
//

use clap;
use log::{info};
use shared::{Command, COMSOCK_PATH};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

// entry point ////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = clap::Command::new("nag")
        .about("Manage nag messages")
        .subcommand(
            clap::Command::new("list")
                .alias("l")
                .about("List all active nags")
        )
        .subcommand(
            clap::Command::new("add")
                .alias("a")
                .about("Adds a new nag")
                .arg(clap::Arg::new("duration").required(true).help("Duration eg: \"1h\" \"2d5h6m3s\""))
                .arg(clap::Arg::new("name").required(true).help("The name for this nag"))
                .arg(clap::Arg::new("sound_file").required(false).help("Path to a sound file to play"))
        )
        .subcommand(
            clap::Command::new("edit")
                .alias("e")
                .about("edits all tags")
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        list_nags().await;
    } else if let Some(add_matches) = matches.subcommand_matches("add") {
        let duration = add_matches.get_one::<String>("duration").unwrap();
        let name = add_matches.get_one::<String>("name").unwrap();
        let sound_file = add_matches.get_one::<String>("sound_file");
        add_nag(duration, name, sound_file).await;
    } else if let Some(_) = matches.subcommand_matches("edit") {
        edit_nags().await;
    }

}

// ----------------------------------------------------------------------------

async fn list_nags() {
    let stream = UnixStream::connect(COMSOCK_PATH).await.expect("Failed to connect");
    let (read_stream, mut write_stream) = stream.into_split();
    let mut reader = BufReader::new(read_stream);

    let command = serde_json::to_string(&Command::ListNags).expect("Failed to convert Command::ListNags to json");
    write_stream.write_all(command.as_bytes()).await.expect("Failed to write");

    let mut response = String::new();
    reader.read_line(&mut response).await.expect("Failed to read response");

    println!("Active Nags: {}", response);
}

// ----------------------------------------------------------------------------

async fn add_nag(duration: &String, name: &String, sound_file: Option<&String>) {
    let stream = UnixStream::connect(COMSOCK_PATH).await.expect("Failed to connect");
    let (read_stream, mut write_stream) = stream.into_split();
    let mut reader = BufReader::new(read_stream);

    let command = serde_json::to_string(&Command::AddNag {
            duration: duration.clone(),
            name: name.clone(),
            sound_file: sound_file.clone().cloned(),
        }).expect("failed to parse add nag command");
    write_stream.write_all(command.as_bytes()).await.expect("Failed to write");

    let mut response = String::new();
    reader.read_line(&mut response).await.expect("Failed to read response");

    println!("Add Nag: {}", response);
}

// ----------------------------------------------------------------------------

async fn edit_nags() {
    println!("NOT YET IMPLEMENTED");
}
