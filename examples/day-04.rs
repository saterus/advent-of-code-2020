extern crate indoc;
extern crate logos;

use logos::{Lexer, Logos};

use advent_of_rust::load_file;

/// Facts listed on a Passport
///
/// * byr (Birth Year)
/// * cid (Country ID)
/// * ecl (Eye Color)
/// * eyr (Expiration Year)
/// * hcl (Hair Color)
/// * hgt (Height)
/// * iyr (Issue Year)
/// * pid (Passport ID)
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Fact<'a> {
    #[regex("byr:([[:alnum:]]+)", fact_value)]
    BirthYear(&'a str),

    #[regex("cid:([#[:alnum:]]+)", fact_value)]
    CountryId(&'a str),

    #[regex("ecl:([[:alnum:]]+)", fact_value)]
    EyeColor(&'a str),

    #[regex("eyr:([[:alnum:]]+)", fact_value)]
    ExpirationYear(&'a str),

    #[regex("hcl:([#[:alnum:]]+)", fact_value)]
    HairColor(&'a str),

    #[regex("hgt:([[:alnum:]]+)", fact_value)]
    Height(&'a str),

    #[regex("iyr:([[:alnum:]]+)", fact_value)]
    IssueYear(&'a str),

    #[regex("pid:([#[:alnum:]]+)", fact_value)]
    PassportId(&'a str),

    #[regex("\n\n+")]
    DocumentEnd,

    #[regex("[^[:space:]]+")]
    Invalid,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

// Note: callbacks can return `Option` or `Result`
fn fact_value<'source>(lex: &mut Lexer<'source, Fact<'source>>) -> &'source str {
    &lex.slice()[4..]
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Passport<'s> {
    birth_year: Option<&'s str>,
    country_id: Option<&'s str>,
    eye_color: Option<&'s str>,
    expiration_year: Option<&'s str>,
    hair_color: Option<&'s str>,
    height: Option<&'s str>,
    issue_year: Option<&'s str>,
    passport_id: Option<&'s str>,
}

impl<'s> Passport<'s> {
    fn is_empty(&self) -> bool {
        [
            self.birth_year,
            self.country_id,
            self.eye_color,
            self.expiration_year,
            self.hair_color,
            self.height,
            self.issue_year,
            self.passport_id,
        ]
        .iter()
        .all(Option::is_none)
    }

    fn is_valid(&self) -> bool {
        [
            self.birth_year,
            // NOT REQUIRED: self.country_id,
            self.eye_color,
            self.expiration_year,
            self.hair_color,
            self.height,
            self.issue_year,
            self.passport_id,
        ]
        .iter()
        .all(Option::is_some)
    }
}

struct PassportParser<'a, 'source: 'a> {
    tokens: &'a mut Lexer<'source, Fact<'source>>,
}

impl<'a, 'source> PassportParser<'a, 'source> {
    fn new(tokens: &'a mut Lexer<'source, Fact<'source>>) -> Self {
        Self { tokens }
    }
}

impl<'a, 'source: 'a> Iterator for PassportParser<'a, 'source> {
    type Item = Passport<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut passport = Passport::default();

        loop {
            match self.tokens.next() {
                None | Some(Fact::DocumentEnd) => {
                    if passport.is_empty() {
                        return None;
                    } else {
                        return Some(passport);
                    }
                }
                Some(Fact::Error) | Some(Fact::Invalid) => {
                    eprintln!(
                        "Encountered an error! Expected a valid token, but found `{}` at `{:?}`",
                        &self.tokens.slice(),
                        self.tokens.span()
                    );
                }
                Some(Fact::BirthYear(year)) => {
                    passport.birth_year = Some(year);
                }
                Some(Fact::CountryId(id)) => {
                    passport.country_id = Some(id);
                }
                Some(Fact::EyeColor(color)) => {
                    passport.eye_color = Some(color);
                }
                Some(Fact::ExpirationYear(year)) => {
                    passport.expiration_year = Some(year);
                }
                Some(Fact::HairColor(color)) => {
                    passport.hair_color = Some(color);
                }
                Some(Fact::Height(measurement)) => {
                    passport.height = Some(measurement);
                }
                Some(Fact::IssueYear(year)) => {
                    passport.issue_year = Some(year);
                }
                Some(Fact::PassportId(id)) => {
                    passport.passport_id = Some(id);
                }
            }
        }
    }
}

fn main() {
    println!("Hello from day-04!");
    let file_contents = load_file("assets/day-04-a.input").expect("Could not read puzzle file!");
    let mut lexer = Fact::lexer(&file_contents);

    let valid_passports = PassportParser::new(&mut lexer)
        .filter(|f| f.is_valid())
        .count();

    println!("Scan found {} valid passports!", valid_passports);
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::*;

    #[test]
    fn multi_passport_parsing_test() {
        let source = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};

        let mut lex = Fact::lexer(source);

        let passports = PassportParser::new(&mut lex)
            .into_iter()
            .collect::<Vec<Passport>>();

        assert_eq!(passports.len(), 4);

        let valid = passports.iter().filter(|f| f.is_valid()).count();
        assert_eq!(valid, 2);
    }

    #[test]
    fn partial_passport_parsing_test() {
        let source = indoc! {"
            ecl:gry pid:860033327 eyr:2020
        "};

        let mut lex = Fact::lexer(source);

        let parser = PassportParser::new(&mut lex);
        let passport = parser.into_iter().next().unwrap();

        assert_eq!(passport.birth_year, None);
        assert_eq!(passport.country_id, None);
        assert_eq!(passport.expiration_year, Some("2020"));
        assert_eq!(passport.eye_color, Some("gry"));
        assert_eq!(passport.hair_color, None);
        assert_eq!(passport.height, None);
        assert_eq!(passport.issue_year, None);
        assert_eq!(passport.passport_id, Some("860033327"));

        assert!(!passport.is_valid());
    }

    #[test]
    fn passport_parsing_test() {
        let source = indoc! {"
            ecl:gry pid:860033327 eyr:2020
            hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm
        "};

        let mut lex = Fact::lexer(source);

        let mut parser = PassportParser::new(&mut lex);
        let passport = parser.next().unwrap();

        assert_eq!(passport.birth_year, Some("1937"));
        assert_eq!(passport.country_id, Some("147"));
        assert_eq!(passport.expiration_year, Some("2020"));
        assert_eq!(passport.eye_color, Some("gry"));
        assert_eq!(passport.hair_color, Some("#fffffd"));
        assert_eq!(passport.height, Some("183cm"));
        assert_eq!(passport.issue_year, Some("2017"));
        assert_eq!(passport.passport_id, Some("860033327"));

        assert!(passport.is_valid());
    }

    #[test]
    fn document_lexing_test() {
        let source = indoc! {"
            ecl:gry pid:860033327
            eyr:2020
            hcl:#fffffd byr:1937

            iyr:2017 cid:147 hgt:183cm


            eyr:2020
        "};

        let mut lex = Fact::lexer(source);

        assert_eq!(lex.next(), Some(Fact::EyeColor("gry")));
        assert_eq!(lex.next(), Some(Fact::PassportId("860033327")));
        assert_eq!(lex.next(), Some(Fact::ExpirationYear("2020")));
        assert_eq!(lex.next(), Some(Fact::HairColor("#fffffd")));
        assert_eq!(lex.next(), Some(Fact::BirthYear("1937")));
        assert_eq!(lex.next(), Some(Fact::DocumentEnd));

        assert_eq!(lex.next(), Some(Fact::IssueYear("2017")));
        assert_eq!(lex.next(), Some(Fact::CountryId("147")));
        assert_eq!(lex.next(), Some(Fact::Height("183cm")));
        assert_eq!(lex.next(), Some(Fact::DocumentEnd));

        assert_eq!(lex.next(), Some(Fact::ExpirationYear("2020")));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn fact_lexing_test() {
        let source = indoc! {"
            ecl:gry pid:860033327 eyr:2020
            hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm
        "};

        let mut lex = Fact::lexer(source);

        assert_eq!(lex.next(), Some(Fact::EyeColor("gry")));
        assert_eq!(lex.next(), Some(Fact::PassportId("860033327")));
        assert_eq!(lex.next(), Some(Fact::ExpirationYear("2020")));
        assert_eq!(lex.next(), Some(Fact::HairColor("#fffffd")));
        assert_eq!(lex.next(), Some(Fact::BirthYear("1937")));
        assert_eq!(lex.next(), Some(Fact::IssueYear("2017")));
        assert_eq!(lex.next(), Some(Fact::CountryId("147")));
        assert_eq!(lex.next(), Some(Fact::Height("183cm")));
        assert_eq!(lex.next(), None);
    }
}
