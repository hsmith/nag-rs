//
// 2024
// SPDX-License-Identifier: MIT
//

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::io::Write;

// config object //////////////////////////////////////////////////////////////

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub edit_tool: Vec<String>,
    pub nag_tool: Vec<String>,
    pub audio_tool: Vec<String>,
}

// ----------------------------------------------------------------------------

impl Default for Config {
    fn default() -> Self {
        Self {
            edit_tool: vec!["nvim".to_string()],
            nag_tool: vec!["i3-nagbar".to_string(), "-m".to_string()],
            audio_tool: vec!["paplay".to_string()],
        }
    }
}

// static config //////////////////////////////////////////////////////////////

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_path = dirs::config_dir()
        .expect("Could not get config_dir")
        .join("nag/config.toml");

    if config_path.exists() {
        let toml_string = std::fs::read_to_string(&config_path).expect("failed to read config");

        toml::from_str(&toml_string).expect("failed to parse toml")
    } else {
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create config_path");
        }

        let default_config = Config::default();
        let toml_string =
            toml::to_string(&default_config).expect("failed to encode default config into toml");
        let mut file = std::fs::File::create(&config_path).expect("failed to create config path");
        file.write_all(toml_string.as_bytes())
            .expect("failed to write default config to config path");

        default_config
    }
});
