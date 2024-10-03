//
// 2024
// SPDX-License-Identifier: MIT
//

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCode {
    OK,
    InvalidFormat,
    UnknownCommand
}
