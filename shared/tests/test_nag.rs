//
// 2024
// SPDX-License-Identifier: MIT
//

use chrono::{Utc, Duration};
use shared::{Nag, time_remaining, nag_to_line, write_nags_to_file, read_nags_from_file};
use std::io::{Cursor, Read, Seek, SeekFrom};

#[test]
fn test_time_remaining() {
    let future_time = Utc::now() + Duration::seconds(86400 + 60*60 + 60 + 2); // 1 day, 1 hour, 1 minute, 1 second (we add an extra second due to inaccuracies with timing)
    let remaining = time_remaining(&future_time);
    assert_eq!(remaining, "1d1h1m1s");

    let past_time = Utc::now() - Duration::seconds(10);
    let remaining = time_remaining(&past_time);
    assert_eq!(remaining, "0");
}

#[test]
fn test_nag_to_line() {
    let nag = Nag {
        end_time: Utc::now(),
        name: "Test Nag".to_string(),
        sound_file: Some("test.wav".to_string()),
    };

    let line = nag_to_line(&nag);
    let expected = format!("\"{}\",\"Test Nag\",\"test.wav\"", nag.end_time.to_rfc3339());
    assert_eq!(line, expected);

    let nag_no_sound = Nag {
        end_time: Utc::now(),
        name: "Silent Nag".to_string(),
        sound_file: None,
    };

    let line_no_sound = nag_to_line(&nag_no_sound);
    let expected_no_sound = format!("\"{}\",\"Silent Nag\"", nag_no_sound.end_time.to_rfc3339());
    assert_eq!(line_no_sound, expected_no_sound);
}

#[test]
fn test_write_nags_to_file() {
    let nag1 = Nag {
        end_time: Utc::now(),
        name: "Nag 1".to_string(),
        sound_file: Some("sound1.wav".to_string()),
    };
    let nag2 = Nag {
        end_time: Utc::now() + Duration::minutes(10),
        name: "Nag 2".to_string(),
        sound_file: None,
    };
    let nags = vec![nag1.clone(), nag2.clone()];

    let mut cursor = Cursor::new(Vec::new());
    write_nags_to_file(&nags, &mut cursor).expect("Failed to write nags to file");

    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut written_data = String::new();
    cursor.read_to_string(&mut written_data).unwrap();

    let expected = format!(
        "{}\n{}\n",
        nag_to_line(&nag1),
        nag_to_line(&nag2)
    );
    assert_eq!(written_data, expected);
}

#[test]
fn test_read_nags_from_file() {
    let nag1 = Nag {
        end_time: Utc::now(),
        name: "Nag 1".to_string(),
        sound_file: Some("sound1.wav".to_string()),
    };
    let nag2 = Nag {
        end_time: Utc::now() + Duration::minutes(10),
        name: "Nag 2".to_string(),
        sound_file: None,
    };

    let data = format!(
        "{}\n{}\n",
        nag_to_line(&nag1),
        nag_to_line(&nag2)
    );

    let mut cursor = Cursor::new(data.into_bytes());
    let read_nags = read_nags_from_file(&mut cursor).expect("Failed to read nags from file");

    assert_eq!(read_nags.len(), 2);
    assert_eq!(read_nags[0], nag1);
    assert_eq!(read_nags[1], nag2);
}

