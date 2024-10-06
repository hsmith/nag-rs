//
// 2024
// SPDX-License-Identifier: MIT
//

pub mod command;
pub mod config;
pub mod error_code;
pub mod nag;
pub mod recv;
pub mod response;
pub mod send;

pub use command::Command;
pub use config::{Config, CONFIG};
pub use error_code::ErrorCode;
pub use nag::{nag_to_line, read_nags_from_file, time_remaining, write_nags_to_file, Nag};
pub use recv::{recv_command, recv_message, recv_response};
pub use response::Response;
pub use send::{send_command, send_message, send_response};

pub const COMSOCK_PATH: &str = "/home/hsmith/.tmp/nag.sock";
