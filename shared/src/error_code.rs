//
// 2024
// SPDX-License-Identifier: MIT
//

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ErrorCode {
    OK,
    InvalidFormat,
    UnknownCommand,
    NotImplemented,
}
