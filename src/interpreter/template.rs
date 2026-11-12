use logos::Logos;
pub type TemplateLine = Token;

type Template = Vec<TemplateLine>;

type Section = String;
#[derive(Debug)]
#[derive(PartialEq)]
pub struct MonthDay {
    pub month: u32,
    pub day: u32,
}

struct OnDate {
    date: MonthDay,
    text: String
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = String)]
enum Token {
    #[token("!copy_tasks", |lex| lex.extras)]
    CopyTasks(Section),
    #[token("!copy_last_day", |lex| lex.extras)]
    CopyLastDay(Section),
    #[regex(r"!by_weekday(\ |\t|\n|\r)*\((.|\n)*?\)", by_weekday)]
    ByWeekday([String; 7]),
    #[regex(r"!on_date(\ |\t|\n|\r)*\((.|\n)*?\)", on_date)]
    OnDate(OnDate),
    #[regex("#.*?\n", header_callback)]
    #[regex(".*?\n", |lex| lex.slice().to_string())]
    Literal(String),
}

fn header_callback(lex: &mut Lexer<Token>){
    lex.extras = lex.slice().to_string();
    lex.slice().to_string()
}

fn by_weekday(lex: &mut Lexer<Token>) -> Token {
    let result = get_content_between_parentheses(line);
    let content_by_day: Vec<String> = result.split(";").map(|s| s.to_string()).collect();
    let content_by_day: [String; 7] = content_by_day.try_into().expect("Expected 7 days");
    Token::ByWeekday(content_by_day)
}

fn on_date(lex: &mut Lexer<Token>) -> Token {
    let result = get_content_between_parentheses(lex.slice());
    let (date_expected, text) = result.split_once(",").unwrap();
    let (month, day) = date_expected
        .trim()
        .split_once("/")
        .expect("Expected month/day format");
    let month: u32 = month.parse().expect("Expected month to be a number");
    let day: u32 = day.parse().expect("Expected day to be a number");
    Token::OnDate(OnDate({
        date: MonthDay{month, day},
        text: text.trim().to_string()
    }))
}

pub fn parse(template: &str) -> Template {
    let mut result = vec![];
    let mut current_section = "".to_string();
    for token in Token::lexer(template) {
        match token {
            Ok(token) => result.push(token),
            Err(_) => continue
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
