use std::fs;
use std::env;
use std::fs::{File, remove_file};
use std::io::prelude::*;
use std::collections::HashMap;
use chrono::{Local, DateTime, Duration, Datelike};  
use std::os::unix::fs as unix_fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = get_config_path(&args);

    let config = get_config(&config_path);

    let current_local: DateTime<Local> = Local::now();  
    create_daily_note(current_local, &config);
    symlink_daily_note(current_local, &config);
}

fn get_config_path(args: &[String]) -> String {
    let mut config_path = "~/.config/groundhog".to_string();

    if args.len() > 1 {
        config_path = args[1].clone();
    }

    if config_path.contains("~") {
        config_path = expand_tilde(&config_path);
    }

    config_path
}

fn expand_tilde(string: &str) -> String {
    let home = env::var("HOME")
        .expect("Stirng contains ~ but variable HOME is not set");
    string.replace("~",&home)
}


fn get_config(config_path: &str) -> HashMap<String, String> {
    let config_path_exists = fs::exists(&config_path)
        .expect("The file system is throwing an error?");

    if !config_path_exists {
        println!(
            "no file found at {}, using default config instead",
            config_path
        );
        return get_default_config();
    }

    let mut config = HashMap::new();
    let config_file = read_file(config_path);


    for line in config_file.lines() {
        let (key, value) = line
             .split_once('=')
             .expect("Invalid config line (missing '=')");

        config.insert(
            key.trim().to_string(),
            expand_tilde(value.trim())
        );
    };
    config
}

fn get_default_config() -> HashMap<String, String> {
    let mut config = HashMap::new();

    config.insert(
        "template_path".to_string(),
        expand_tilde("~/Notes/templates/dnt.md"));
    config.insert(
        "symlink_path".to_string(),
        expand_tilde("~/Notes/daily-note.md"));
    config.insert(
        "archive_path".to_string(),
        expand_tilde("~/Notes/daily-note/"));

    config
}

fn create_daily_note(date: DateTime<Local>, config: &HashMap<String, String>) {
    if note_exists(date, config) {
        println!("Daily note {} already exists", date_to_file_name(date, config));
        return
    }

    let yesterday = date - Duration::days(1);
    create_daily_note(yesterday, config);

    let yesterday = date_to_file_name(yesterday, config);
    let yesterday = read_file(&yesterday);

    let template_path: String = config.get("template_path")
        .expect("Base path not provided on config").to_owned().to_string();
    let template = read_file(&template_path);

    let note = process_tokens(&template, &yesterday, date);
    let path = date_to_file_name(date, config);

    let file = File::create(path);
    let _ = file.expect("File createon failed").write_all(note.as_bytes());
}

fn note_exists(date: DateTime<Local>, config: &HashMap<String, String>) -> bool {
    let note_name = date_to_file_name(date, config);
    fs::exists(note_name).expect("The file system is throwing an error?")
}

fn date_to_file_name(date: DateTime<Local>, config: &HashMap<String, String>) -> String {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let file_name = format!("{}-{:0>2}-{:0>2}.md", year, month, day);
    let archive_path: String = config.get("archive_path")
        .expect("Base path not provided on config").to_owned().to_string();
    archive_path + &file_name
}

fn read_file(path: &str) -> String {
    let file = File::open(path);
    let mut contents = String::new();
    let _ = file.expect("Failed to read file {}").read_to_string(&mut contents);
    contents
} 

fn process_tokens(template: &str, yesterday: &str, date: DateTime<Local>) -> String {
    let mut result = "".to_string();
    let mut current_section = "";
    let dont_copy = ["- [x]"];
    for line in template.lines() {
        if line.starts_with("#") {
            current_section = line;
        }
        if line.contains("!copy_last_day") {
            result += &get_section_text(current_section, yesterday, &dont_copy);
            result += "\n";
            continue
        }
        if line.starts_with("!by_weekday") {
            result += &by_weekday(line, date);
            result += "\n";
            continue
        }
        result += line;
        result += "\n";
    }
    result
}
fn get_section_text(section: &str, yesterday: &str, dont_copy: &[&str; 1]) -> String {
    let mut result = "".to_string();
    let mut copy = false;
    for line in yesterday.lines() {
        if line.starts_with("#") {
            copy = false;
        }
        if copy & !should_skip(line, dont_copy) {
            result += line;
            result += "\n";
        }
        if line.starts_with(section) {
            copy = true;
        }
    }
    result.trim().to_string()
}

fn should_skip(line: &str, dont_copy: &[&str; 1]) -> bool {
    for token in dont_copy {
        if line.starts_with(token) {
            return true
        }
    };
    false
}

fn by_weekday(line: &str, date: DateTime<Local>) -> String{
    let start_bytes = line
        .find("[").expect("no opening brackets found in by weekday template line");
    let end_bytes = line.find("]")
        .expect("no closing brackets found in by weekday template line");
    let result = &line[start_bytes+1..end_bytes];
    let content_by_day: Vec<&str> = result.split(";").collect();
    let index = date.weekday().num_days_from_sunday();
    let index = usize::try_from(index).expect("Cannot convert weekday to usize");
    content_by_day[index].to_string()
}

fn symlink_daily_note(date: DateTime<Local>, config: &HashMap<String, String>) {
    println!("creating symlink for daily-note: {}", date_to_file_name(date, config));
    let daily_note = date_to_file_name(date, config);
    let sym_link_path: String = config.get("symlink_path")
        .expect("Base path not provided on config").to_owned().to_string();
    let _ = remove_file(&sym_link_path);
    let _ = unix_fs::symlink(daily_note, sym_link_path);
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

        let result  = process_tokens(&template, &yesterday, date);

        let expected = read_file("test_data/expected.md");
        assert_eq!(result, expected);
    }
}
