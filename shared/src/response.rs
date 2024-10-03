//
// 2024
// SPDX-License-Identifier: MIT
//

use crate::error_code::ErrorCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub code: ErrorCode,
    pub payload: Option<String>,
}
