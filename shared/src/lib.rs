//
// 2024
// SPDX-License-Identifier: MIT
//

pub mod command;
pub mod error_code;
pub mod response;

pub use command::Command;
pub use error_code::ErrorCode;
pub use response::Response;

pub const COMSOCK_PATH : &str = "/home/hsmith/.tmp/nag.sock";
