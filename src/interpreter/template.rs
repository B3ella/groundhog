use logos::Logos;
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

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("!copy_tasks")]
    CopyTasks,
    #[token("!copy_last_day")]
    CopyLastDay,
    #[regex(r"!by_weekday(\ |\t|\n|\r)*\((.|\n)*?\))", by_weekday)]
    ByWeekday([String; 7]),
    #[regex(r"!on_date(\ |\t|\n|\r)*\((.|\n)*?\))", on_date)]
    OnDate(MonthDay, String),
    #[regex(".*?\n", |lex| lex.slice().to_string())]
    Literal(String),
}

fn by_weekday(lex: &mut Lexer<Token>){
    let result = get_content_between_parentheses(line);
    let content_by_day: Vec<String> = result.split(";").map(|s| s.to_string()).collect();
    let content_by_day: [String; 7] = content_by_day.try_into().expect("Expected 7 days");
    Token::ByWeekday(content_by_day)
}

fn on_date(lex: &mut Lexer<Token>) -> {
    let result = get_content_between_parentheses(lex.slice());
    let (date_expected, text) = result.split_once(",").unwrap();
    let (month, day) = date_expected
        .trim()
        .split_once("/")
        .expect("Expected month/day format");
    let month: u32 = month.parse().expect("Expected month to be a number");
    let day: u32 = day.parse().expect("Expected day to be a number");
    Token::OnDate(MonthDay { month, day }, text.trim().to_string())
}

pub fn parse(template: &str) -> Template {
    let mut result = vec![];
    let mut current_section = "".to_string();
    for token in Token::lexer(template) {
        match token.unwrap() {
            Token::Literal(line) => {
                if line.starts_with("#") {
                    current_section = line.trim().to_string()
                }
                result.push(TemplateLine::Literal(line.to_string()))
            }
            Token::CopyTasks => TemplateLine::CopyTasks(current_section),
            Token::CopyLastDay => TemplateLine::CopyLastDay(current_section),
            Token::ByWeekday(content_by_day) => TemplateLine::ByWeekday(content_by_day),
            Token::OnDate(day, text) => TemplateLine::OnDate(day, text),
        }
    }
    result
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
