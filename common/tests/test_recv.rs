//
// 2024
// SPDX-License-Identifier: MIT
//

use common::command::Command;
use common::response::Response;
use common::{recv_command, recv_message, recv_response};
use tokio::io::{self, AsyncWriteExt};

#[tokio::test]
async fn test_recv_message() {
    let (mut write_stream, mut read_stream) = io::duplex(64);

    // Write test data to the stream
    let msg = "Hello, World!".to_string();
    write_stream.write_u32_le(msg.len() as u32).await.unwrap();
    write_stream.write_all(msg.as_bytes()).await.unwrap();

    // Read the message using recv_message
    let received_msg = recv_message(&mut read_stream).await.unwrap();
    assert_eq!(received_msg, msg);
}

#[tokio::test]
async fn test_recv_command() {
    let (mut write_stream, mut read_stream) = io::duplex(128);

    let command = Command::ListNags;

    // Write the serialized command to the stream
    let serialized_command = serde_json::to_string(&command).unwrap();
    write_stream
        .write_u32_le(serialized_command.len() as u32)
        .await
        .unwrap();
    write_stream
        .write_all(serialized_command.as_bytes())
        .await
        .unwrap();

    // Read the command using recv_command
    let received_command = recv_command(&mut read_stream).await.unwrap();
    assert_eq!(received_command, command);
}

#[tokio::test]
async fn test_recv_response() {
    let (mut write_stream, mut read_stream) = io::duplex(128);

    let response = Response::Ok;

    // Write the serialized response to the stream
    let serialized_response = serde_json::to_string(&response).unwrap();
    write_stream
        .write_u32_le(serialized_response.len() as u32)
        .await
        .unwrap();
    write_stream
        .write_all(serialized_response.as_bytes())
        .await
        .unwrap();

    // Read the response using recv_response
    let received_response = recv_response(&mut read_stream).await.unwrap();
    assert_eq!(received_response, response);
}
