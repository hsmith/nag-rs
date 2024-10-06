//
// 2024
// SPDX-License-Identifier: MIT
//

use crate::error_code::ErrorCode;
use crate::nag::Nag;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Response {
    Ok,
    Error { code: ErrorCode, msg: Option<String> },
    NagList { nags: Vec<Nag> },
}
