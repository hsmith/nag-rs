//
// 2024
// SPDX-License-Identifier: MIT
//

use chrono::{DateTime, Utc};
use duration_str::parse;
use log::{info, error};
use serde::{Serialize, Deserialize};
use shared::{Command, ErrorCode, Response, COMSOCK_PATH};
use std::fs;
use std::path::Path;
use std::sync::{Arc};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::time::{ Duration, interval };

// Nag data structure /////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone)]
struct Nag {
    end_time: DateTime<Utc>,
    name: String,
    sound_file: Option<String>,
}

// NagList ////////////////////////////////////////////////////////////////////

type NagList = Arc<Mutex<Vec<Nag>>>;

// ensure dir /////////////////////////////////////////////////////////////////

fn ensure_dir(file_path: &str) {
    let path = Path::new(file_path);
    let parent_dir = path.parent().expect("invalid path");
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).expect("Failed to create directory");
    }
}

// entry point/////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting nagd...");
    ensure_dir(COMSOCK_PATH);
    let nags = NagList::new(Mutex::new(Vec::new()));
    
    let connections_clone = Arc::clone(&nags);
    spawn(handle_connections(connections_clone));

    let process_clone = Arc::clone(&nags);
    process_nags(process_clone).await;

} 

// ----------------------------------------------------------------------------

async fn process_nags(nags: NagList) {
    let mut interval = interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        let mut nags_lock = nags.lock().await;
        let now = Utc::now();

        // find all nags that are finished
        nags_lock.retain(|nag| {
            if nag.end_time <= now {
                let nag_clone = nag.clone();
                tokio::spawn(async move {
                    trigger_nag(nag_clone).await;
                });
                return false;
            }
            else {
                return true;
            }
        });
    }
}

// ----------------------------------------------------------------------------

async fn trigger_nag(nag: Nag) {
    println!("Trigger Nag! {}", nag.name);
}

// ----------------------------------------------------------------------------

async fn handle_connections(nag_list: NagList) {
    // clean up any existing socket
    let socket_path = COMSOCK_PATH;
    let _ = std::fs::remove_file(socket_path);

    loop {
        let nags = Arc::clone(&nag_list);

        info!("Waiting for connection...");
        let listener = UnixListener::bind(socket_path).expect("failed to bind socket");
        let (stream, _) = listener.accept().await.expect("Listener failed to accept");
        let (read_stream, mut write_stream) = stream.into_split();

        let mut reader = BufReader::new(read_stream);
        let mut line = String::new();

        reader.read_line(&mut line).await.expect("Failed to read line");
        info!("Received command...");

        let response = match serde_json::from_str::<Command>(&line) {
            Ok(command)=> match command {
                Command::AddNag { duration, name, sound_file } => add_nag(duration, name, sound_file, &nags).await,
                Command::ListNags => list_nags(nags).await,
            }
            Err(err)=> Response {
                code: ErrorCode::UnknownCommand,
                payload: Some(err.to_string()),
            },
        };

        info!("Sending response...");
        let response_json = serde_json::to_string(&response).expect(&format!("Failed to encode {:?} to json", response));
        if let Err(err) = write_stream.write_all(response_json.as_bytes()).await {
            error!("Failed to write to stream with error: {}", err);
        }
    }
}

// ----------------------------------------------------------------------------

async fn add_nag(duration: String, name: String, sound_file: Option<String>, nags: &NagList) -> Response {
    let mut nags = nags.lock().await;

    match parse(&duration) {
        Ok(duration_parsed) => {
            nags.push(Nag {
                end_time: Utc::now() + duration_parsed,
                name,
                sound_file
            });

            Response {
                code: ErrorCode::OK,
                payload: None
            }
        },
        Err(err) => Response {
            code: ErrorCode::InvalidFormat,
            payload: Some(format!("Failed to add nage {}", err)),
        },
    }
}

// ----------------------------------------------------------------------------

async fn list_nags(nags: NagList) -> Response {
    let nags = nags.lock().await;
    let nags_list = serde_json::to_string(&*nags).unwrap_or_else(|_| "[]".to_string());

    Response {
        code: ErrorCode::OK,
        payload: Some(nags_list)
    }
}
