//
// 2024
// SPDX-License-Identifier: MIT
//

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    AddNag { duration: String, name: String, sound_file: Option<String> },
    ListNags,
}
