//
// 2024
// SPDX-License-Identifier: MIT
//

use chrono::Utc;
use common::{
    read_nags_from_file, recv_response, send_command, write_nags_to_file, Command, Nag, Response,
    COMSOCK_PATH, CONFIG,
};
use std::env;
use std::fs;
use std::io::{Write};
use std::path::Path;
use log::info;
use std::process::Command as Proc;
use tempfile::NamedTempFile;
use tokio::net::UnixStream;

// entry point ////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = clap::Command::new("nag")
        .about("Manage nag messages")
        .subcommand(clap::Command::new("list").about("List all active nags"))
        .subcommand(
            clap::Command::new("add")
                .about("Adds a new nag")
                .arg(
                    clap::Arg::new("duration")
                        .required(true)
                        .help("Duration eg: \"1h\" \"2d5h6m3s\""),
                )
                .arg(
                    clap::Arg::new("name")
                        .required(true)
                        .help("The name for this nag"),
                )
                .arg(
                    clap::Arg::new("sound_file")
                        .required(false)
                        .help("Path to a sound file to play"),
                ),
        )
        .subcommand(
            clap::Command::new("edit")
                .about("edits all tags"),
        )
        .get_matches();

    if matches.subcommand_matches("list").is_some() {
        list_nags().await;
    } else if let Some(add_matches) = matches.subcommand_matches("add") {
        let duration = add_matches.get_one::<String>("duration").unwrap();
        let name = add_matches.get_one::<String>("name").unwrap();
        let sound_file = add_matches.get_one::<String>("sound_file");
        add_nag(duration, name, sound_file).await;
    } else if matches.subcommand_matches("edit").is_some() {
        edit_nags().await;
    }
}

// ----------------------------------------------------------------------------

async fn fetch_nags() -> Result<Vec<Nag>, Box<(dyn std::error::Error + Sync + Send)>> {
    info!("Connecting to socket...");
    let stream = UnixStream::connect(COMSOCK_PATH)
        .await
        .expect("Failed to connect");
    let (mut read_stream, mut write_stream) = stream.into_split();

    info!("Connected, sending list_nags command");
    send_command(&mut write_stream, Command::ListNags)
        .await
        .expect("Failed to send command");

    info!("Command sent, waiting reponse...");
    match recv_response(&mut read_stream)
        .await
        .expect("Failed to recv response")
    {
        Response::Ok => Ok(vec![]),
        Response::NagList { nags } => Ok(nags),
        Response::Error { code, msg } => {
            let error_message = format!(
                "Error! {} ({:?})",
                msg.unwrap_or("No Message".to_string()),
                code
            );
            Err(Box::<dyn std::error::Error + Sync + Send>::from(
                error_message,
            ))
        }
    }
}

// ----------------------------------------------------------------------------

async fn list_nags() {
    match fetch_nags().await {
        Ok(nags) => {
            println!("{:?}", nags);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

// ----------------------------------------------------------------------------

async fn add_nag(duration: &String, name: &str, sound_file: Option<&String>) {
    let stream = UnixStream::connect(COMSOCK_PATH)
        .await
        .expect("Failed to connect");
    let (mut read_stream, mut write_stream) = stream.into_split();

    let nag = match duration_str::parse(duration) {
        Ok(duration_parsed) => Nag {
            end_time: Utc::now() + duration_parsed,
            name: name.to_string(),
            sound_file: sound_file.cloned(),
        },
        Err(err) => panic!("Failed to parse duration {} ({:?})", duration, err),
    };

    send_command(&mut write_stream, Command::AddNag { nag })
        .await
        .expect("Failed to send command to add nag");

    info!("Command sent, waiting response...");
    let response = recv_response(&mut read_stream)
        .await
        .expect("Failed to recv response");

    match response {
        Response::Ok => println!("Success"),
        Response::NagList { nags } => println!("Nags: {:?}", nags),
        Response::Error { code, msg } => println!(
            "Error! {} ({:?})",
            msg.as_deref().unwrap_or("No Message"),
            code
        ),
    }
}

// ----------------------------------------------------------------------------

async fn send_nags(nags: Vec<Nag>) -> Result<Vec<Nag>, Box<dyn std::error::Error + Sync + Send>> {
    let stream = UnixStream::connect(COMSOCK_PATH)
        .await
        .expect("Failed to connect");
    let (mut read_stream, mut write_stream) = stream.into_split();

    info!("Sending nags {:?}", nags);
    send_command(&mut write_stream, Command::SetNags { nags })
        .await
        .expect("failed send set nags command");

    info!("Command sent, waiting response...");
    let response = recv_response(&mut read_stream)
        .await
        .expect("Failed to recv response");

    match response {
        Response::Ok => Ok(Vec::new()),
        Response::NagList { nags } => Ok(nags),
        Response::Error { code, msg } => {
            Err(Box::<dyn std::error::Error + Sync + Send>::from(format!(
                "Error! {} ({:?})",
                msg.as_deref().unwrap_or("No Message"),
                code
            )))
        }
    }
}

// ----------------------------------------------------------------------------

async fn edit_nags() {
    // fetch all nags
    let nags = fetch_nags().await.expect("failed to fetch nags");

    // write all nags to a temporary file converting the first column
    // from a utc time stamp to a duration string from now()
    let mut temp_file = NamedTempFile::new().expect("Failed to open temp file");
    write_nags_to_file(&nags, &mut temp_file).expect("Failed to write nags to file");

    // run nvim on the temp file and wait for it to close
    let mut proc = Proc::new(&CONFIG.edit_tool[0]);
    for arg in &CONFIG.edit_tool[1..] {
        proc.arg(arg);
    }

    let status = proc
        .arg(temp_file.path())
        .status()
        .expect("Failed to start nvim");

    if !status.success() {
        eprintln!("something went wrong with nvim");
        return;
    }

    // read the nags back in from the temp file
    let new_nags = read_nags_from_file(&mut temp_file).expect("failed to read nags");

    // compare the new nags to the old nags, if they are different, send the new
    // nags to be stored
    if nags != new_nags {
        send_nags(new_nags).await.expect("failed to send nags");
    } else {
        info!("Nags ARE the same, nothing to do");
    }
}
