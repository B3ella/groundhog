use chrono::{DateTime, Datelike, Local};

pub fn process_tokens(template: &str, yesterday: &str, date: DateTime<Local>) -> String {
    let mut result = "".to_string();
    let mut current_section = "";
    for line in template.lines() {
        let is_header = line.starts_with("#");
        if is_header {
            current_section = line;
        }
        let token_end = line.find(" ").unwrap();
        let token = &line[..token_end];
        result += match token {
            "!copy_tasks" => &get_section_text(current_section, yesterday, &vec!["- [x]"]),
            "!copy_last_day" => &get_section_text(current_section, yesterday, &vec![]),
            "!by_weekday" => &by_weekday(line, date),
            _ => line
        };
        result += "\n";
    }
    result.trim().to_string()
}

fn get_section_text(section: &str, yesterday: &str, dont_copy: &Vec<&str>) -> String {
    let mut result = "".to_string();
    let mut copy = false;

    let header_end = section.find(" ").unwrap();
    let header = &section[..header_end+1];

    let lines = yesterday.lines().filter(|line| !should_skip(line, dont_copy));
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

fn should_skip(line: &str, dont_copy: &Vec<&str>) -> bool {
    for token in dont_copy {
        if line.starts_with(token) {
            return true;
        }
    }
    false
}

fn by_weekday(line: &str, date: DateTime<Local>) -> String {
    let start_bytes = line
        .find("[")
        .expect("no opening brackets found in by weekday template line");
    let end_bytes = line
        .find("]")
        .expect("no closing brackets found in by weekday template line");
    let result = &line[start_bytes + 1..end_bytes];
    let content_by_day: Vec<&str> = result.split(";").collect();
    let index = date.weekday().num_days_from_sunday();
    let index = usize::try_from(index).expect("Cannot convert weekday to usize");
    content_by_day[index].to_string()
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
                    "!by_weekday[Sunday;Monday;Tuesday;Wednesday;Thursday;Friday;Saturday]",
                    date(year, month, day)
                ),
                value
            );
        }
    }

    #[test]
    fn process_tokens_replaces_weekday_and_preserves_regular_lines() {
        let output = process_tokens(
            "# Day\n!by_weekday[Sunday;Monday;Tuesday;Wednesday;Thursday;Friday;Saturday]\nKeep this",
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
        yesterday, date(2024, 1, 8));

        assert_eq!(output, 
            "# Work\n- unfinished\n## Subsection\n# Personal"
        );
    }

    #[test]
    fn copy_tasks_skips_completed_items_but_keeps_open_items() {
        let yesterday = "# Tasks\n- [x] done\n- [ ] pending\n# Other\n- unrelated";
        let output = process_tokens("# Tasks\n!copy_tasks", yesterday, date(2024, 1, 8));
        assert_eq!(output, "# Tasks\n- [ ] pending");
    }
}
