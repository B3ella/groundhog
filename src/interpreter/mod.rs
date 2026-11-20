mod template;
use chrono::{DateTime, Datelike, Local};

pub fn process_tokens(template: &str, yesterday: &str, date: DateTime<Local>) -> String {
    let mut result = String::new();
    let template = template::parse(template);
    for line in template {
        result += &process_token(&line, yesterday, date);
    }
    result.trim().to_string()
}

fn process_token(
    content: &template::TemplateLine,
    yesterday: &str,
    date: DateTime<Local>,
) -> String {
    match content {
        template::TemplateLine::CopyTasks(section) => copy_tasks(yesterday, section),
        template::TemplateLine::CopyLastDay(section) => get_section_text(section, yesterday),
        template::TemplateLine::ByWeekday(content_by_day) => by_weekday(content_by_day, date),
        template::TemplateLine::OnDate(params) => on_date(&params.date, &params.text, date),
        template::TemplateLine::Literal(literal) => literal.clone(),
    }
}

fn copy_tasks(yesterday: &str, current_section: &str) -> String {
    get_section_text(current_section, yesterday)
        .lines()
        .filter(|line| !line.starts_with("- [x]"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_section_text(section: &str, yesterday: &str) -> String {
    let section_start = match yesterday.find(section) {
        Some(v) => v + section.len(),
        None => return String::new(),
    };

    let header_end = section.find(' ').unwrap_or(0);
    let header = "\n".to_string() + &section[..=header_end];
    let section_end = match yesterday[section_start..].find(&header) {
        None => yesterday.len(),
        Some(v) => v + section_start,
    };

    let section_text = &yesterday[section_start..section_end];
    section_text.trim().to_string()
}

fn by_weekday(content_by_day: &[String; 7], date: DateTime<Local>) -> String {
    let index = date.weekday().num_days_from_sunday();
    let index = usize::try_from(index).expect("Cannot convert weekday to usize");
    content_by_day.get(index).unwrap_or(&String::new()).clone()
}

fn on_date(expected_date: &template::MonthDay, text: &str, real_date: DateTime<Local>) -> String {
    if real_date.month() == expected_date.month && real_date.day() == expected_date.day {
        return text.to_string();
    }
    String::new()
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
        let template = "!on_date(20/11, test text)";
        let output = process_tokens(template, "test text", date(2024, 20, 11));
        assert_eq!(output, "test text");
    }
    #[test]
    fn on_date_doesnt_return_on_wrong_date() {
        let template = "no text\n!on_date(01/01, test text)";
        let output = process_tokens(template, "", date(2024, 1, 2));
        assert_eq!(output, "no text");
    }
}
