//
// 2024
// SPDX-License-Identifier: MIT
//

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ErrorCode {
    OK,
    InvalidFormat,
    UnknownCommand,
    NotImplemented,
}
