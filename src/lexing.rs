use std::collections::BTreeMap;
use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub(crate) enum TokenType {
    Attribute,
    DefaultAttribute,
    EmptyMilestone,
    StartMilestoneTag,
    EndMilestoneTag,
    EndTag,
    StartTag,
    BareSlash,
    Eol,
    NoBreakSpace,
    SoftLineBreak,
    SpaceCharacters,
    NonSpaceCharacters,
    Main,
}

#[derive(Clone)]
pub(crate) struct RegexRecord {
    pub(crate) re_string: String,
    pub(crate) re: Regex
}

pub(crate) fn lexing_regexes() -> BTreeMap<TokenType, RegexRecord> {
    let mut regexes = BTreeMap::new();
    regexes.insert(
        TokenType::Attribute,
        RegexRecord {
            re_string: r#"([ \t]*\|?[ \t]*([A-Za-z0-9\-]+)="([^"]*)"[ \t]?)"#.to_string(),
            re: Regex::new(r#"([ \t]*\|?[ \t]*([A-Za-z0-9\-]+)="([^"]*)"[ \t]?)"#).unwrap()
        }
    );
    regexes.insert(
        TokenType::DefaultAttribute,
        RegexRecord {
            re_string: r"([ \t]*\|[ \t]*([^|\\]*))".to_string(),
            re: Regex::new(r"([ \t]*\|[ \t]*([^|\\]*))").unwrap()
        }
    );
    regexes.insert(
        TokenType::EmptyMilestone,
        RegexRecord {
            re_string: r"(\\([a-z1-9]+)\\[*])".to_string(),
            re: Regex::new(r"(\\([a-z1-9]+)\\[*])").unwrap()
        }
    );
    regexes.insert(
        TokenType::StartMilestoneTag,
        RegexRecord {
            re_string: r"(\\([a-z1-9]+)-([se]))".to_string(),
            re: Regex::new(r"(\\([a-z1-9]+)-([se]))").unwrap()
        }
    );
    regexes.insert(
        TokenType::EndMilestoneTag,
        RegexRecord {
            re_string: r"(\\([*]))".to_string(),
            re: Regex::new(r"(\\([*]))").unwrap()
        }
    );
    regexes.insert(
        TokenType::EndTag,
        RegexRecord {
            re_string: r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[*])".to_string(),
            re: Regex::new(r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[*])").unwrap()
        }
    );
    regexes.insert(
        TokenType::StartTag,
        RegexRecord {
            re_string: r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?))".to_string(),
            re: Regex::new(r"(\\([+]?[a-z\-]+)([1-9]?(-([1-9]))?)[ \t]?)").unwrap()
        }
    );
    regexes.insert(
        TokenType::BareSlash,
        RegexRecord {
            re_string: r"(\\)".to_string(),
            re: Regex::new(r"(\\)").unwrap()
        }
    );
    regexes.insert(
        TokenType::Eol,
        RegexRecord {
            re_string: r"([ \t]*[\r\n]+[ \t]*)".to_string(),
            re: Regex::new(r"([ \t]*[\r\n]+[ \t]*)").unwrap()
        }
    );
    regexes.insert(
        TokenType::NoBreakSpace,
        RegexRecord {
            re_string: r"~".to_string(),
            re: Regex::new(r"~").unwrap()
        }
    );
    regexes.insert(
        TokenType::SpaceCharacters,
        RegexRecord {
            re_string: r"(\p{Zs}{1,1024})".to_string(),
            re: Regex::new(r"(\p{Zs}{1,1024})").unwrap()
        }
    );
    regexes.insert(
        TokenType::NonSpaceCharacters,
        RegexRecord {
            re_string: r"([^\p{Zs}]{1,1024})".to_string(),
            re: Regex::new(r"([^\p{Zs}]{1,1024})").unwrap()
        }
    );

    let main_regex_str: String = regexes.iter()
        .map(|(_, v)| v.re_string.clone())
        .collect::<Vec<_>>()
        .join("|");
    let main_regex = Regex::new(&main_regex_str).unwrap();
    regexes.insert(TokenType::Main, RegexRecord {re_string: main_regex_str, re: main_regex});
    regexes
}