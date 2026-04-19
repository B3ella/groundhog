use std::fs;
use std::fs::{File, remove_file};
use std::io::prelude::*;
use chrono::{Local, DateTime, Duration, Datelike};  
use std::os::unix::fs as unix_fs;

const BASE_PATH: &str = "/home/bella/notes";

fn main() {
    let current_local: DateTime<Local> = Local::now();  
    create_daily_note(current_local);
    symlink_daily_note(current_local);
}

fn create_daily_note(date: DateTime<Local>) {
    if note_exists(date) {
        println!("Daily note {} already exists", date_to_file_name(date));
        return
    }

    let yesterday = date - Duration::days(1);
    create_daily_note(yesterday);

    let yesterday = date_to_file_name(yesterday);
    let yesterday = read_file(&yesterday);

    let template = read_file(&(BASE_PATH.to_owned() + "templates/dnt.md"));

    let note = process_tokens(&template, &yesterday, date);
    let path = date_to_file_name(date);

    let file = File::create(path);
    let _ = file.expect("File createon failed").write_all(note.as_bytes());
}

fn note_exists(date: DateTime<Local>) -> bool {
    let note_name = date_to_file_name(date);
    let result = fs::exists(note_name).expect("The file system is throwing an error?");
    return result
}

fn date_to_file_name(date: DateTime<Local>) -> String {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let file_name = format!("{}-{:0>2}-{:0>2}.md", year, month, day);
    BASE_PATH.to_owned() + &file_name
}

fn read_file(path: &str) -> String {
    let file = File::open(path);
    let mut contents = String::new();
    let _ = file.expect("Failed to read file {}").read_to_string(&mut contents);
    return contents
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
            result += &get_section_text(&current_section, &yesterday, &dont_copy);
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
    return result
}
fn get_section_text(section: &str, yesterday: &str, dont_copy: &[&str; 1]) -> String {
    let mut result = "".to_string();
    let mut copy = false;
    for line in yesterday.lines() {
        if line.starts_with("#") {
            copy = false;
        }
        if copy &! should_skip(line, dont_copy) {
            result += line;
            result += "\n";
        }
        if line.starts_with(&section) {
            copy = true;
        }
    }
    return result.trim().to_string()
}

fn should_skip(line: &str, dont_copy: &[&str; 1]) -> bool {
    for token in dont_copy {
        if line.starts_with(token) {
            return true
        };
    };
    return false
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
    return content_by_day[index].to_string();
}

fn symlink_daily_note(date: DateTime<Local>) {
    println!("creating symlink for daily-note: {}", date_to_file_name(date));
    let daily_note = date_to_file_name(date);
    let sym_link_path = BASE_PATH.to_owned() + "daily-note.md";
    let _ = remove_file("a.txt");
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
