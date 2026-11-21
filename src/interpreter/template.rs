use logos::{Lexer, Logos};
pub type TemplateLine = Token;

type Template = Vec<TemplateLine>;

type Section = String;
#[derive(Debug, PartialEq, Eq)]
pub struct MonthDay {
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OnDate {
    pub date: MonthDay,
    pub text: String,
}

#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(extras = String)]
#[logos(subpattern whitespace = r"(\ |\t|\n|\r)")]
#[logos(subpattern params = r"(?&whitespace)*\((.|\n)*?\)")]
pub enum Token {
    #[regex(r"!copy_last_day", |lex| lex.extras.clone())]
    CopyLastDay(Section),
    #[regex(r"!copy_tasks", |lex| lex.extras.clone())]
    CopyTasks(Section),
    #[regex(r"!by_weekday(?&params)", by_weekday)]
    ByWeekday([String; 7]),
    #[regex(r"!on_date(?&params)", on_date)]
    OnDate(OnDate),
    #[regex(r"#+\ .+?\n", header_callback)]
    #[regex(r".|\n", |lex| lex.slice().to_string())]
    Literal(String),
}

fn header_callback(lex: &mut Lexer<Token>) -> String {
    lex.extras = lex.slice().trim().to_string();
    lex.slice().to_string()
}

fn by_weekday(lex: &Lexer<Token>) -> [String; 7] {
    let result = get_content_between_parentheses(lex.slice());
    let content_by_day: Vec<String> = result
        .split(';')
        .map(std::string::ToString::to_string)
        .collect();
    let content_by_day: [String; 7] = content_by_day.try_into().expect("Expected 7 days");
    content_by_day
}

fn on_date(lex: &Lexer<Token>) -> OnDate {
    let result = get_content_between_parentheses(lex.slice());
    let (date_expected, text) = result.split_once(',').unwrap();
    let (day, month) = date_expected
        .trim()
        .split_once('/')
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
    for token in Token::lexer(template).flatten() {
        result.push(token)
    }
    result
}

fn get_content_between_parentheses(line: &str) -> String {
    let start_bytes = line
        .find('(')
        .expect("no opening brackets found in by weekday template line");
    let end_bytes = line
        .find(')')
        .expect("no closing brackets found in by weekday template line");
    let result = &line[start_bytes + 1..end_bytes];
    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_between_parenthesis() {
        assert_eq!(
            get_content_between_parentheses("teste(test)teste"),
            "test".to_string()
        )
    }

    #[test]
    fn can_parse_by_weekday() {
        let expected = vec![Token::ByWeekday([
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
        ])];
        assert_eq!(parse("!by_weekday(1;2;3;4;5;6;7)"), expected)
    }
    #[test]
    fn can_parse_copy_last_day() {
        let expected = vec![
            Token::Literal("# sec\n".to_string()),
            Token::CopyLastDay("# sec".to_string()),
        ];
        assert_eq!(parse("# sec\n!copy_last_day"), expected)
    }

    #[test]
    fn can_parse_copy_tasks() {
        let expected = vec![
            Token::Literal("# sec\n".to_string()),
            Token::CopyTasks("# sec".to_string()),
        ];
        assert_eq!(parse("# sec\n!copy_tasks"), expected)
    }
    #[test]
    fn can_parse_on_date() {
        let expected = Token::OnDate(OnDate {
            date: MonthDay { month: 11, day: 20 },
            text: "test".to_string(),
        });
        assert_eq!(parse("!on_date(20/11, test)"), vec![expected])
    }
    #[test]
    fn can_parse_literal() {
        let expected = vec![
            Token::Literal("L".to_string()),
            Token::Literal("i".to_string()),
            Token::Literal("t".to_string()),
            Token::Literal("e".to_string()),
            Token::Literal("r".to_string()),
            Token::Literal("a".to_string()),
            Token::Literal("l".to_string()),
        ];
        assert_eq!(parse("Literal"), expected)
    }
}
