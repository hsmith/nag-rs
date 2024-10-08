//
// 2024
// SPDX-License-Identifier: MIT
//

use chrono::{DateTime, Duration, Utc};
use duration_str::parse;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

// Nag data structure /////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Nag {
    pub end_time: DateTime<Utc>,
    pub name: String,
    pub sound_file: Option<String>,
}

// ----------------------------------------------------------------------------

#[must_use]
pub fn time_remaining(end_time: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = *end_time - now;

    if duration < Duration::zero() {
        return "0".to_string();
    }

    let mut remaining = String::new();
    let seconds = duration.num_seconds();

    let days = seconds / (24 * 60 * 60);
    if days > 0 {
        remaining.push_str(&format!("{}d", days));
    }

    let hours_left = seconds - (days * 24 * 60 * 60);
    let hours = hours_left / (60 * 60);
    if hours > 0 {
        remaining.push_str(&format!("{}h", hours));
    }

    let minutes_left = hours_left - (hours * 60 * 60);
    let minutes = minutes_left / 60;
    if minutes > 0 {
        remaining.push_str(&format!("{}m", minutes));
    }

    let seconds_left = minutes_left - (minutes * 60);
    if seconds_left > 0 {
        remaining.push_str(&format!("{}s", seconds_left));
    }

    remaining
}

// ----------------------------------------------------------------------------

#[must_use]
pub fn nag_to_line(nag: &Nag) -> String {
    if let Some(sound_file) = &nag.sound_file {
        format!(
            "\"{}\",\"{}\",\"{}\"",
            nag.end_time.to_rfc3339(),
            nag.name,
            sound_file
        )
    } else {
        format!("\"{}\",\"{}\"", nag.end_time.to_rfc3339(), nag.name)
    }
}

// ----------------------------------------------------------------------------

pub fn write_nags_to_file<W: Write + Seek>(nags: &Vec<Nag>, writer: &mut W) -> io::Result<()> {
    writer
        .seek(SeekFrom::End(0))
        .expect("Failed to seek to end of file");
    for nag in nags {
        let line = nag_to_line(nag);
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

// ----------------------------------------------------------------------------

pub fn read_nags_from_file<R: Read + Seek>(
    read: &mut R,
) -> Result<Vec<Nag>, Box<dyn std::error::Error>> {
    read.seek(SeekFrom::Start(0))
        .expect("failed to seek to beginning of file");
    let reader = io::BufReader::new(read);

    let mut nags = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            if !line.is_empty() {
                eprintln!(
                    "Skipping malformed line {line}, parts len: {}, parts: {parts:?}",
                    parts.len()
                );
            }
            continue;
        }

        let s = parts[0].trim_matches('"');
        let end_time = match DateTime::parse_from_rfc3339(s) {
            Ok(datetime) => datetime.with_timezone(&Utc),
            Err(_) => {
                println!("NOT a date time, lets try parsing...");
                Utc::now() + parse(s).expect("Failed to parse string")
            }
        };
        let name = parts[1].trim_matches('"').to_string();
        let sound_file = if parts.len() > 2 && parts[2] != "None" {
            Some(parts[2].trim_matches('"').to_string())
        } else {
            None
        };

        nags.push(Nag {
            end_time,
            name,
            sound_file,
        });
    }

    Ok(nags)
}
