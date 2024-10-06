//
// 2024
// SPDX-License-Identifier: MIT
//

use shared::send::{send_command, send_message, send_response};
use shared::command::Command;
use shared::response::Response;

use tokio::io::{self, AsyncReadExt};

#[tokio::test]
async fn test_send_message() {
    let (mut write_stream, mut read_stream) = io::duplex(64);
    
    let msg = "Hello, World!".to_string();
    let send_result = send_message(&mut write_stream, msg.clone()).await;
    
    assert!(send_result.is_ok());

    let mut len_buf = [0u8; 4];
    read_stream.read_exact(&mut len_buf).await.unwrap();
    let msg_len = u32::from_le_bytes(len_buf) as usize;

    let mut read_buf = vec![0u8; msg_len];
    read_stream.read_exact(&mut read_buf).await.unwrap();
    
    assert_eq!(String::from_utf8(read_buf).unwrap(), msg);
}

#[tokio::test]
async fn test_send_command() {
    let (mut write_stream, mut read_stream) = io::duplex(128);
    
    let command = Command::ListNags;
    
    let send_result = send_command(&mut write_stream, command.clone()).await;
    assert!(send_result.is_ok());

    let mut len_buf = [0u8; 4];
    read_stream.read_exact(&mut len_buf).await.unwrap();
    let msg_len = u32::from_le_bytes(len_buf) as usize;

    let mut read_buf = vec![0u8; msg_len];
    read_stream.read_exact(&mut read_buf).await.unwrap();

    let received_command: Command = serde_json::from_slice(&read_buf).unwrap();
    assert_eq!(received_command, command);
}

#[tokio::test]
async fn test_send_response() {
    let (mut write_stream, mut read_stream) = io::duplex(128);

    let response = Response::Ok;

    let send_result = send_response(&mut write_stream, response.clone()).await;
    assert!(send_result.is_ok());

    let mut len_buf = [0u8; 4];
    read_stream.read_exact(&mut len_buf).await.unwrap();
    let msg_len = u32::from_le_bytes(len_buf) as usize;

    let mut read_buf = vec![0u8; msg_len];
    read_stream.read_exact(&mut read_buf).await.unwrap();

    let received_response: Response = serde_json::from_slice(&read_buf).unwrap();
    assert_eq!(received_response, response);
}

