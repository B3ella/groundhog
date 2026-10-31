use chrono::{Local, DateTime, Datelike};  

pub fn process_tokens(template: &str, yesterday: &str, date: DateTime<Local>) -> String {
    let mut result = "".to_string();
    let mut current_section = "";
    for line in template.lines() {
        if line.starts_with("#") {
            current_section = line;
        }
        if line.contains("!copy_last_day") {
            result += &get_section_text(current_section, yesterday, &[]);
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