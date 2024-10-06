//
// 2024
// SPDX-License-Identifier: MIT
//

pub mod command;
pub mod error_code;
pub mod nag;
pub mod response;
pub mod recv;
pub mod send;

pub use command::Command;
pub use error_code::ErrorCode;
pub use nag::{Nag, time_remaining, nag_to_line, write_nags_to_file, read_nags_from_file};
pub use response::Response;
pub use recv::{recv_message, recv_command, recv_response};
pub use send::{send_message, send_command, send_response};

pub const COMSOCK_PATH : &str = "/home/hsmith/.tmp/nag.sock";
