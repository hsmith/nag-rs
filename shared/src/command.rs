//
// 2024
// SPDX-License-Identifier: MIT
//

use crate::nag::Nag;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    AddNag { nag: Nag },
    ListNags,
    SetNags { nags : Vec<Nag> },
}


