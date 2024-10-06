//
// 2024
// SPDX-License-Identifier: MIT
//

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum ErrorCode {
    OK,
    InvalidFormat,
    UnknownCommand,
    NotImplemented,
}
