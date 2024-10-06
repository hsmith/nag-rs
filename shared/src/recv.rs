//
// 2024
// SPDX-License-Identifier: MIT
//

use crate::command::Command;
use crate::response::Response;

use tokio::io::AsyncReadExt;

///////////////////////////////////////////////////////////////////////////////

pub async fn recv_message<R>(
    reader: &mut R,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>>
where
    R: AsyncReadExt + Unpin,
{
    let size = reader.read_u32_le().await?;

    let mut buf = vec![0; size as usize];
    reader.read_exact(&mut buf).await?;

    let message = String::from_utf8(buf)?;
    Ok(message)
}

// ----------------------------------------------------------------------------

pub async fn recv_response<R>(
    reader: &mut R,
) -> Result<Response, Box<dyn std::error::Error + Sync + Send>>
where
    R: AsyncReadExt + Unpin,
{
    let message = recv_message(reader).await?;
    Ok(serde_json::from_str::<Response>(&message)?)
}

// ----------------------------------------------------------------------------

pub async fn recv_command<R>(
    reader: &mut R,
) -> Result<Command, Box<dyn std::error::Error + Sync + Send>>
where
    R: AsyncReadExt + Unpin,
{
    let message = recv_message(reader).await?;
    Ok(serde_json::from_str::<Command>(&message)?)
}
