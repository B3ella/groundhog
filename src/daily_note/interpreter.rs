use chrono::{DateTime, Datelike, Local};

type Template = Vec<TemplateLine>;
enum TemplateLine {
    copy_tasks,
    copy_last_day,
    by_weekday([String; 7]),
    on_date(MonthDay, String),
    literal(String),
    heading(String),
}
struct MonthDay {
    month: u32,
    day: u32,
}

pub fn process_tokens(template: &str, yesterday: &str, date: DateTime<Local>) -> String {
    let mut result = "".to_string();
    let mut current_section = "".to_string();
    let template = get_template(template);
    for line in template {
        if let TemplateLine::heading(ref text) = line {
            current_section = text.to_string();
        }
        result += &process_token(&line, yesterday, &current_section, date);
        result += "\n";
    }
    result.trim().to_string()
}

fn get_template(template: &str) -> Template {
    let mut result = vec![];
    for line in template.lines() {
        result.push(get_content(line));
    }
    result
}
fn get_content(line: &str) -> TemplateLine {
    let token = get_token(line);
    match token.as_str() {
        "!copy_tasks" => TemplateLine::copy_tasks,
        "!copy_last_day" => TemplateLine::copy_last_day,
        "!by_weekday" => {
            let result = get_content_between_parentheses(line);
            let content_by_day: Vec<String> = result.split(";").map(|s| s.to_string()).collect();
            let content_by_day: [String; 7] = content_by_day.try_into().expect("Expected 7 days");
            TemplateLine::by_weekday(content_by_day)
        }
        "!on_date" => {
            let result = get_content_between_parentheses(line);
            let (date_expected, text) = result.split_once(",").unwrap();
            let (month, day) = date_expected
                .trim()
                .split_once("/")
                .expect("Expected month/day format");
            let month: u32 = month.parse().expect("Expected month to be a number");
            let day: u32 = day.parse().expect("Expected day to be a number");
            TemplateLine::on_date(MonthDay { month, day }, text.trim().to_string())
        }
        _ => {
            if line.starts_with("#") {
                TemplateLine::heading(line.to_string())
            } else {
                TemplateLine::literal(line.to_string())
            }
        }
    }
}
fn get_token(line: &str) -> String {
    let token_end = line.find("(");
    let token = match token_end {
        Some(token_end) => line[..token_end].trim(),
        None => line,
    };
    token.to_string()
}

fn process_token(
    content: &TemplateLine,
    yesterday: &str,
    current_section: &str,
    date: DateTime<Local>,
) -> String {
    match content {
        TemplateLine::copy_tasks => get_section_text(current_section, yesterday, &vec!["- [x]"]),
        TemplateLine::copy_last_day => get_section_text(current_section, yesterday, &vec![]),
        TemplateLine::by_weekday(content_by_day) => by_weekday(content_by_day, date),
        TemplateLine::on_date(expectedDate, text) => on_date(expectedDate, text, date),
        TemplateLine::literal(literal) => literal.to_string(),
        TemplateLine::heading(heading) => heading.to_string(),
    }
}

fn get_section_text(section: &str, yesterday: &str, dont_copy: &Vec<&str>) -> String {
    let mut result = "".to_string();
    let mut copy = false;

    let header_end = section.find(" ").unwrap();
    let header = &section[..header_end + 1];

    let lines = yesterday
        .lines()
        .filter(|line| !should_skip(line, dont_copy));
    for line in lines {
        if line.starts_with(header) {
            copy = false;
        }
        if copy {
            result += line;
            result += "\n";
        }
        if line.starts_with(section) {
            copy = true;
        }
    }
    result.trim().to_string()
}

fn by_weekday(content_by_day: &[String; 7], date: DateTime<Local>) -> String {
    let index = date.weekday().num_days_from_sunday();
    let index = usize::try_from(index).expect("Cannot convert weekday to usize");
    content_by_day[index].to_string()
}

fn on_date(expectedDate: &MonthDay, text: &str, realDate: DateTime<Local>) -> String {
    if realDate.month() == expectedDate.month && realDate.day() == expectedDate.day {
        return text.to_string();
    }
    "".to_string()
}

fn should_skip(line: &str, dont_copy: &Vec<&str>) -> bool {
    for token in dont_copy {
        if line.starts_with(token) {
            return true;
        }
    }
    false
}

fn get_content_between_parentheses(line: &str) -> String {
    let start_bytes = line
        .find("(")
        .expect("no opening brackets found in by weekday template line");
    let end_bytes = line
        .find(")")
        .expect("no closing brackets found in by weekday template line");
    let result = &line[start_bytes + 1..end_bytes];
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn date(year: i32, month: u32, day: u32) -> DateTime<Local> {
        Local
            .with_ymd_and_hms(year, month, day, 12, 0, 0)
            .single()
            .unwrap()
    }

    #[test]
    fn weekday_token_selects_sunday_through_saturday() {
        let expected = [
            (2024, 1, 7, "Sunday"),
            (2024, 1, 8, "Monday"),
            (2024, 1, 9, "Tuesday"),
            (2024, 1, 10, "Wednesday"),
            (2024, 1, 11, "Thursday"),
            (2024, 1, 12, "Friday"),
            (2024, 1, 13, "Saturday"),
        ];

        for (year, month, day, value) in expected {
            assert_eq!(
                by_weekday(
                    &[
                        "Sunday".to_string(),
                        "Monday".to_string(),
                        "Tuesday".to_string(),
                        "Wednesday".to_string(),
                        "Thursday".to_string(),
                        "Friday".to_string(),
                        "Saturday".to_string()
                    ],
                    date(year, month, day)
                ),
                value
            );
        }
    }

    #[test]
    fn process_tokens_replaces_weekday_and_preserves_regular_lines() {
        let output = process_tokens(
            "# Day\n!by_weekday(Sunday;Monday;Tuesday;Wednesday;Thursday;Friday;Saturday)\nKeep this",
            "",
            date(2024, 1, 8),
        );
        assert_eq!(output, "# Day\nMonday\nKeep this");
    }

    #[test]
    fn copy_last_day_copies_the_matching_section_until_the_next_heading() {
        let yesterday = "# Work\n- unfinished\n## Subsection\n# Personal\n- unrelated";

        let output = process_tokens(
            "# Work\n!copy_last_day\n# Personal",
            yesterday,
            date(2024, 1, 8),
        );

        assert_eq!(output, "# Work\n- unfinished\n## Subsection\n# Personal");
    }

    #[test]
    fn copy_tasks_skips_completed_items_but_keeps_open_items() {
        let yesterday = "# Tasks\n- [x] done\n- [ ] pending\n# Other\n- unrelated";
        let output = process_tokens("# Tasks\n!copy_tasks", yesterday, date(2024, 1, 8));
        assert_eq!(output, "# Tasks\n- [ ] pending");
    }
    #[test]
    fn on_date_returns_content_on_date() {
        let template = "!on_date(01/01, test text)";
        let output = process_tokens(template, "", date(2024, 1, 1));
        assert_eq!(output, "test text");
    }
    #[test]
    fn on_date_doesnt_return_on_wrong_date() {
        let template = "no text\n!on_date(01/01, test text)";
        let output = process_tokens(template, "", date(2024, 1, 2));
        assert_eq!(output, "no text");
    }
}
