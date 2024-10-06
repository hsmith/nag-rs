//
// 2024
// SPDX-License-Identifier: MIT
//

use crate::command::Command;
use crate::response::Response;

use tokio::io::AsyncWriteExt;

///////////////////////////////////////////////////////////////////////////////

pub async fn send_message<W>(
    write_stream: &mut W,
    msg: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    W: AsyncWriteExt + Unpin,
{
    write_stream.write_u32_le(msg.len() as u32).await?;
    println!("sending message ({})...", msg);
    write_stream.write_all(msg.as_bytes()).await?;

    Ok(())
}
// ----------------------------------------------------------------------------

pub async fn send_command<W>(
    write_stream: &mut W,
    command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    W: AsyncWriteExt + Unpin,
{
    let message = serde_json::to_string(&command)?;
    send_message(write_stream, message).await?;

    Ok(())
}

// ----------------------------------------------------------------------------

pub async fn send_response<W>(
    write_stream: &mut W,
    response: Response,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    W: AsyncWriteExt + Unpin,
{
    let message = serde_json::to_string(&response)?;
    send_message(write_stream, message).await?;

    Ok(())
}
