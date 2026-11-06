use crate::config;
use chrono::{DateTime, Datelike, Local};
use std::fs;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::os::unix::fs as unix_fs;
mod interpreter;
use regex::Regex;
use std::cmp;

pub fn symlink_daily_note(date: DateTime<Local>, config: &config::Config) {
    println!(
        "creating symlink for daily-note: {}",
        date_to_file_name(date, config)
    );
    let daily_note = date_to_file_name(date, config);
    let _ = remove_file(&config.symlink_path);
    let _ = unix_fs::symlink(daily_note, &config.symlink_path);
}

pub fn create_daily_note(date: DateTime<Local>, config: &config::Config) {
    let path = date_to_file_name(date, config);

    if note_exists(date, config) {
        println!("Daily note {} already exists", path);
        return;
    }

    let last_note = get_latest_note(&config.archive_path);

    let last_note = match last_note {
        Some(note) => read_file(&note),
        None => "".to_string(),
    };

    let template = read_file(&config.template_path);

    let note = interpreter::process_tokens(&template, &last_note, date);

    let file = File::create(path);
    let _ = file
        .expect("File createon failed")
        .write_all(note.as_bytes());
}

fn note_exists(date: DateTime<Local>, config: &config::Config) -> bool {
    let note_name = date_to_file_name(date, config);
    fs::exists(note_name).expect("The file system is throwing an error?")
}

fn date_to_file_name(date: DateTime<Local>, config: &config::Config) -> String {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let file_name = format!("{}-{:0>2}-{:0>2}.md", year, month, day);
    config.archive_path.clone() + &file_name
}

fn read_file(path: &str) -> String {
    let file = File::open(path);
    let mut contents = String::new();
    let _ = file
        .expect("Failed to read file {}")
        .read_to_string(&mut contents);
    contents
}

fn get_latest_note(path: &str) -> Option<String> {
    let files = fs::read_dir(path).ok()?;
    let paths = files.map(|file| {
        file.expect("File system error")
            .path()
            .to_string_lossy()
            .to_string()
    });

    let daily_note_reggex = Regex::new(r".*\d{4}-\d{2}-\d{2}.md").unwrap();
    let daily_notes = paths.filter(|file| daily_note_reggex.is_match(file));

    let mut max = "".to_string();
    for note in daily_notes {
        let string_path = note;
        max = cmp::max(string_path, max);
    }
    if max.is_empty() {
        return None;
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "groundhog-daily-note-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn date(year: i32, month: u32, day: u32) -> DateTime<Local> {
        Local
            .with_ymd_and_hms(year, month, day, 12, 0, 0)
            .single()
            .unwrap()
    }

    fn config_for(dir: &std::path::Path) -> config::Config {
        config::Config {
            template_path: dir.join("template.md").to_string_lossy().into_owned(),
            symlink_path: dir.join("today.md").to_string_lossy().into_owned(),
            archive_path: format!("{}/", dir.to_string_lossy()),
        }
    }

    #[test]
    fn date_filename_uses_zero_padded_month_and_day() {
        let dir = temp_dir();
        let cfg = config_for(&dir);

        assert_eq!(
            date_to_file_name(date(2024, 2, 3), &cfg),
            format!("{}/2024-02-03.md", dir.display())
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn create_greenfield_note() {
        let dir = temp_dir();
        let cfg = config_for(&dir);

        fs::write(&cfg.template_path, "# Work\n!copy_last_day\n").unwrap();
        create_daily_note(date(2024, 2, 2), &cfg);

        let expected = "# Work";
        assert_eq!(
            fs::read_to_string(date_to_file_name(date(2024, 2, 2), &cfg)).unwrap(),
            expected
        );

        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn creates_note_using_template_and_yesterdays_section() {
        let dir = temp_dir();
        let cfg = config_for(&dir);

        fs::write(&cfg.template_path, "# Work\n!copy_last_day\n").unwrap();
        fs::write(
            date_to_file_name(date(2024, 2, 1), &cfg),
            "# Work\n- carry me\n",
        )
        .unwrap();

        create_daily_note(date(2024, 2, 2), &cfg);

        let expected = "# Work\n- carry me";
        assert_eq!(
            fs::read_to_string(date_to_file_name(date(2024, 2, 2), &cfg)).unwrap(),
            expected
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn creates_note_based_on_arbitrarily_old_notes() {
        let dir = temp_dir();
        let cfg = config_for(&dir);

        fs::write(&cfg.template_path, "# Work\n!copy_last_day\n").unwrap();
        fs::write(
            date_to_file_name(date(2022, 2, 1), &cfg),
            "# Work\n- carry me\n",
        )
        .unwrap();

        create_daily_note(date(2024, 2, 2), &cfg);

        let expected = "# Work\n- carry me";
        assert_eq!(
            fs::read_to_string(date_to_file_name(date(2024, 2, 2), &cfg)).unwrap(),
            expected
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn existing_note_is_not_overwritten() {
        let dir = temp_dir();
        let cfg = config_for(&dir);
        let existing = date_to_file_name(date(2024, 2, 2), &cfg);

        fs::write(&existing, "keep this note").unwrap();

        create_daily_note(date(2024, 2, 2), &cfg);

        assert_eq!(fs::read_to_string(existing).unwrap(), "keep this note");

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn symlink_points_to_the_requested_daily_note() {
        let dir = temp_dir();
        let cfg = config_for(&dir);
        let note = date_to_file_name(date(2024, 2, 2), &cfg);

        fs::write(&note, "today").unwrap();

        symlink_daily_note(date(2024, 2, 2), &cfg);

        assert_eq!(
            fs::read_link(&cfg.symlink_path).unwrap(),
            std::path::PathBuf::from(note)
        );

        fs::remove_dir_all(dir).unwrap();
    }
}
