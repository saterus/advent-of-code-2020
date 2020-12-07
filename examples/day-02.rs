extern crate logos;

use logos::Lexer;
use logos::Logos;

use advent_of_rust::load_file;

#[derive(Logos, Debug, PartialEq)]
enum PasswordRuleToken<'a> {
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(u64),

    #[token("-")]
    Dash,

    #[regex("[a-z]:", |lex| lex.slice().chars().next())]
    TargetCharacter(char),

    #[regex("[a-z]+", |lex| lex.slice())]
    Password(&'a str),

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug, PartialEq, Eq)]
struct PasswordRule<'l> {
    first_spot: usize,
    second_spot: usize,
    target_char: char,
    password: &'l str,
}

impl<'l> PasswordRule<'l> {
    fn is_valid(&self) -> bool {
        let first_spot = self.password.chars().nth(self.first_spot);
        let second_spot = self.password.chars().nth(self.second_spot);

        match (first_spot, second_spot) {
            (Some(x), Some(y)) if x == self.target_char && y == self.target_char => false,
            (Some(x), _) if x == self.target_char => true,
            (_, Some(x)) if x == self.target_char => true,
            _ => false,
        }
    }
}

struct Parser<'p, 'l: 'p> {
    lexer: &'p mut Lexer<'l, PasswordRuleToken<'l>>,
}

impl<'p, 'l: 'p> Parser<'p, 'l> {
    fn new(lexer: &'p mut Lexer<'l, PasswordRuleToken<'l>>) -> Self {
        Self { lexer }
    }

    fn parse_rule<'a>(&'a mut self) -> Result<PasswordRule<'l>, String>
    where
        'p: 'a,
    {
        let first_spot = if let Some(PasswordRuleToken::Number(n)) = self.lexer.next() {
            (n - 1) as usize // these numbers represent one-based indexes
        } else {
            return Err("Expected the first password rule number!".to_string());
        };

        if let Some(PasswordRuleToken::Dash) = self.lexer.next() {
            // good parse
        } else {
            return Err("Expected the dash!".to_string());
        };

        let second_spot = if let Some(PasswordRuleToken::Number(n)) = self.lexer.next() {
            (n - 1) as usize // these numbers represent one-based indexes
        } else {
            return Err("Expected the second password rule number!".to_string());
        };

        let target_char =
            if let Some(PasswordRuleToken::TargetCharacter(target)) = self.lexer.next() {
                target
            } else {
                return Err("Expected the required target character!".to_string());
            };

        let password = if let Some(PasswordRuleToken::Password(password)) = self.lexer.next() {
            password
        } else {
            return Err("Expected the password itself!".to_string());
        };

        Ok(PasswordRule {
            first_spot: first_spot,
            second_spot: second_spot,
            target_char,
            password,
        })
    }
}

struct ParserIntoIter<'p, 'l> {
    parser: Parser<'p, 'l>,
}

impl<'p, 'l: 'p> Iterator for ParserIntoIter<'p, 'l> {
    type Item = PasswordRule<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(password_rule) = self.parser.parse_rule() {
            Some(password_rule)
        } else {
            None
        }
    }
}

impl<'p, 'l> IntoIterator for Parser<'p, 'l> {
    type Item = PasswordRule<'l>;
    type IntoIter = ParserIntoIter<'p, 'l>;

    fn into_iter(self) -> Self::IntoIter {
        ParserIntoIter { parser: self }
    }
}

fn main() {
    println!("Hello from day-02!");

    let file_contents = load_file("assets/day-02-a.input").expect("Could not read puzzle file!");
    let mut lexer = PasswordRuleToken::lexer(&file_contents);
    let parser = Parser::new(&mut lexer);

    let rules = parser.into_iter().collect::<Vec<PasswordRule>>();

    let total_rules = rules.len();

    let valid_passwords = rules.iter().filter(|rule| rule.is_valid());

    println!(
        "There were {}/{} valid passwords.",
        valid_passwords.count(),
        total_rules
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parser_iter_test() {
        let mut lex = PasswordRuleToken::lexer("1-3 a: abcde\n2-4 b: cdefg\n");
        let parser = Parser::new(&mut lex);
        let mut iter = parser.into_iter();

        let rule = iter.next().expect("first rule");
        assert_eq!(rule.target_char, 'a');

        let rule2 = iter.next().expect("second rule");
        assert_eq!(rule2.target_char, 'b');

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn basic_password_rule_test() -> Result<(), String> {
        let mut lex = PasswordRuleToken::lexer("1-3 a: abcde\n2-4 b: cdefg\n");
        let mut parser = Parser::new(&mut lex);

        let rule = parser.parse_rule()?;

        assert_eq!(rule.first_spot, 1);
        assert_eq!(rule.second_spot, 3);
        assert_eq!(rule.target_char, 'a');
        assert_eq!(rule.password, "abcde");

        let rule2 = parser.parse_rule()?;

        assert_eq!(rule2.first_spot, 2);
        assert_eq!(rule2.second_spot, 4);
        assert_eq!(rule2.target_char, 'b');
        assert_eq!(rule2.password, "cdefg");
        Ok(())
    }

    #[test]
    fn basic_lexing_test() {
        let mut lex = PasswordRuleToken::lexer("1-3 a: abcde");

        assert_eq!(lex.next(), Some(PasswordRuleToken::Number(1)));
        assert_eq!(lex.span(), 0..1);
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), Some(PasswordRuleToken::Dash));
        assert_eq!(lex.span(), 1..2);
        assert_eq!(lex.slice(), "-");

        assert_eq!(lex.next(), Some(PasswordRuleToken::Number(3)));
        assert_eq!(lex.span(), 2..3);
        assert_eq!(lex.slice(), "3");

        assert_eq!(lex.next(), Some(PasswordRuleToken::TargetCharacter('a')));
        assert_eq!(lex.span(), 4..6);
        assert_eq!(lex.slice(), "a:");

        assert_eq!(lex.next(), Some(PasswordRuleToken::Password("abcde")));
        assert_eq!(lex.span(), 7..12);
        assert_eq!(lex.slice(), "abcde");
    }

    #[test]
    fn second_basic_lexing_test() {
        let lex = PasswordRuleToken::lexer("1-3 b: cdefg");

        let tokens = lex.collect::<Vec<PasswordRuleToken>>();
        assert_eq!(
            tokens,
            vec![
                PasswordRuleToken::Number(1),
                PasswordRuleToken::Dash,
                PasswordRuleToken::Number(3),
                PasswordRuleToken::TargetCharacter('b'),
                PasswordRuleToken::Password("cdefg")
            ]
        );
    }
}
