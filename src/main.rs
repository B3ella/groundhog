use std::fs;
use std::env;
use std::fs::{File, remove_file};
use std::io::prelude::*;
use chrono::{Local, DateTime, Duration, Datelike};  
use std::os::unix::fs as unix_fs;
mod config;
mod interpreter;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = config::get_config_path(&args);

    let config = config::get_config(&config_path);

    let current_local: DateTime<Local> = Local::now();  
    create_daily_note(current_local, &config, config.max_recursion);
    symlink_daily_note(current_local, &config);
}

fn create_daily_note(date: DateTime<Local>, config: &config::Config, max_recursion: i32) {
    let path = date_to_file_name(date, config);

    if note_exists(date, config) {
        println!("Daily note {} already exists", &path);
        return
    }

    if max_recursion < 0 {
        println!("hit max recursion, creating blank note as start");
        let file = File::create(&path);
        let _ = file.expect("File creation failed")
            .write_all("".as_bytes());
        return
    }

    let yesterday = date - Duration::days(1);
    create_daily_note(yesterday, config, max_recursion - 1);

    let yesterday = date_to_file_name(yesterday, config);
    let yesterday = read_file(&yesterday);

    let template = read_file(&config.template_path);

    let note = interpreter::process_tokens(&template, &yesterday, date);

    let file = File::create(path);
    let _ = file.expect("File createon failed").write_all(note.as_bytes());
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
    let _ = file.expect("Failed to read file {}").read_to_string(&mut contents);
    contents
} 

fn symlink_daily_note(date: DateTime<Local>, config: &config::Config) {
    println!("creating symlink for daily-note: {}", date_to_file_name(date, config));
    let daily_note = date_to_file_name(date, config);
    let _ = remove_file(&config.symlink_path);
    let _ = unix_fs::symlink(daily_note, &config.symlink_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let template = read_file("test_data/template.md");
        let yesterday = read_file("test_data/yesterday.md");

        let date = "2026-03-18T12:12:12Z"
            .parse::<DateTime<Local>>()
            .expect("Error converting date to local");

        let result = interpreter::process_tokens(&template, &yesterday, date);

        let expected = read_file("test_data/expected.md");
        assert_eq!(result, expected);
    }
}
