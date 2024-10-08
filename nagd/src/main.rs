//
// 2024
// SPDX-License-Identifier: MIT
//

use chrono::Utc;
use common::{
    recv_command, send_response, Command, ErrorCode, Nag, Response, COMSOCK_PATH, CONFIG,
};
use log::info;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::net::UnixListener;
use tokio::process::Command as Proc;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

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
                false
            } else {
                true
            }
        });
    }
}

// ----------------------------------------------------------------------------

async fn trigger_nag(nag: Nag) {
    let nagbar = tokio::spawn(async move {
        let mut proc = Proc::new(&CONFIG.nag_tool[0]);
        for arg in &CONFIG.nag_tool[1..] {
            proc.arg(arg);
        }
        proc.status().await.expect("failed to execute i3-navbar");
    });

    let paplay = nag.sound_file.map(|sfx_file| {
        let mut proc = Proc::new(&CONFIG.audio_tool[0]);
        for arg in &CONFIG.audio_tool[1..] {
            proc.arg(arg);
        }
        proc.arg(sfx_file);

        proc.spawn().expect("failed to execute audio_tool")
    });

    nagbar.await.expect("nag_tool finished successfully");
    if let Some(mut child) = paplay {
        if let Err(e) = child.kill().await {
            eprintln!("Failed to kill paplay child with error {}", e);
        }
    } else {
        info!("No paplay");
    }
}

// ----------------------------------------------------------------------------

async fn handle_connections(nag_list: NagList) {
    // clean up any existing socket
    let socket_path = COMSOCK_PATH;
    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).expect("failed to bind socket");

    loop {
        let nags = Arc::clone(&nag_list);

        info!("Socket bound, waiting connection...");
        let (stream, _) = listener.accept().await.expect("Listener failed to accept");

        let (read_stream, mut write_stream) = stream.into_split();
        let mut reader = BufReader::new(read_stream);
        info!("Connection joined!  Awaiting command...");

        let response = match recv_command(&mut reader).await {
            Ok(command) => match command {
                Command::AddNag { nag } => add_nag(nag, &nags).await,
                Command::ListNags => list_nags(nags).await,
                Command::SetNags { nags: new_nags } => set_nags(new_nags, &nags).await,
            },
            Err(err) => Response::Error {
                code: ErrorCode::UnknownCommand,
                msg: Some(err.to_string()),
            },
        };

        info!("Sending response...");
        send_response(&mut write_stream, response)
            .await
            .expect("Failed to send response");
    }
}

// ----------------------------------------------------------------------------

async fn add_nag(nag: Nag, nags: &NagList) -> Response {
    let mut nags = nags.lock().await;
    nags.push(nag);
    Response::Ok
}

// ----------------------------------------------------------------------------

async fn list_nags(nags: NagList) -> Response {
    let nags = nags.lock().await;
    let nags_list = serde_json::to_string(&*nags).unwrap_or_else(|_| "[]".to_string());

    info!("Listing nags... {}", nags_list);
    Response::NagList { nags: nags.clone() }
}

// ----------------------------------------------------------------------------

async fn set_nags(new_nags: Vec<Nag>, nags: &NagList) -> Response {
    let mut nags = nags.lock().await;
    *nags = new_nags;
    Response::Ok
}
