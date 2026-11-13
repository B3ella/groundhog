use logos::{Lexer, Logos};
pub type TemplateLine = Token;

type Template = Vec<TemplateLine>;

type Section = String;
#[derive(Debug, PartialEq)]
pub struct MonthDay {
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, PartialEq)]
pub struct OnDate {
    pub date: MonthDay,
    pub text: String,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = String)]
#[logos(subpattern whitespace = r"(\ |\t|\n|\r)")]
#[logos(subpattern params = r"(?&whitespace)*\((.|\n)*?\)")]
pub enum Token {
    #[regex(r"!copy_last_day", |lex| lex.extras.to_string())]
    CopyLastDay(Section),
    #[regex(r"!copy_tasks", |lex| lex.extras.to_string())]
    CopyTasks(Section),
    #[regex(r"!by_weekday(?&params)", by_weekday)]
    ByWeekday([String; 7]),
    #[regex(r"!on_date(?&params)", on_date)]
    OnDate(OnDate),
    #[regex(r"#+\ .+?\n", header_callback)]
    #[regex(r".+?(?&whitespace)", |lex| lex.slice().to_string())]
    Literal(String),
}

fn header_callback(lex: &mut Lexer<Token>) -> String {
    lex.extras = lex.slice().trim().to_string();
    lex.slice().to_string()
}

fn by_weekday(lex: &mut Lexer<Token>) -> [String; 7] {
    let result = get_content_between_parentheses(lex.slice());
    let content_by_day: Vec<String> = result.split(";").map(|s| s.to_string()).collect();
    let content_by_day: [String; 7] = content_by_day.try_into().expect("Expected 7 days");
    content_by_day
}

fn on_date(lex: &mut Lexer<Token>) -> OnDate {
    let result = get_content_between_parentheses(lex.slice());
    let (date_expected, text) = result.split_once(",").unwrap();
    let (month, day) = date_expected
        .trim()
        .split_once("/")
        .expect("Expected month/day format");
    let month: u32 = month.parse().expect("Expected month to be a number");
    let day: u32 = day.parse().expect("Expected day to be a number");
    OnDate {
        date: MonthDay { month, day },
        text: text.trim().to_string(),
    }
}

pub fn parse(template: &str) -> Template {
    let mut result = vec![];
    for token in Token::lexer(template) {
        match token {
            Ok(token) => result.push(token),
            Err(_) => continue,
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