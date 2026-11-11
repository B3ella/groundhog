pub enum TemplateLine {
    CopyTasks(Section),
    CopyLastDay(Section),
    ByWeekday([String; 7]),
    OnDate(MonthDay, String),
    Literal(String),
}

type Template = Vec<TemplateLine>;

type Section = String;
pub struct MonthDay {
    pub month: u32,
    pub day: u32,
}

pub fn parse(template: &str) -> Template {
    let mut result = vec![];
    let mut current_section = "".to_string();
    for line in template.lines() {
        if line.starts_with("#") {
            current_section = line.to_string()
        }
        result.push(get_content(line, &current_section));
    }
    result
}
fn get_content(line: &str, current_section: &str) -> TemplateLine {
    let token = get_token(line);
    match token.as_str() {
        "!copy_tasks" => TemplateLine::CopyTasks(current_section.to_string()),
        "!copy_last_day" => TemplateLine::CopyLastDay(current_section.to_string()),
        "!by_weekday" => {
            let result = get_content_between_parentheses(line);
            let content_by_day: Vec<String> = result.split(";").map(|s| s.to_string()).collect();
            let content_by_day: [String; 7] = content_by_day.try_into().expect("Expected 7 days");
            TemplateLine::ByWeekday(content_by_day)
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
            TemplateLine::OnDate(MonthDay { month, day }, text.trim().to_string())
        }
        _ => TemplateLine::Literal(line.to_string()),
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

