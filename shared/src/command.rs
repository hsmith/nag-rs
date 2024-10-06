//
// 2024
// SPDX-License-Identifier: MIT
//

use crate::nag::Nag;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Command {
    AddNag { nag: Nag },
    ListNags,
    SetNags { nags: Vec<Nag> },
}
