use std::collections::BTreeMap;
use std::io::BufRead;
use std::fs::File;
use std::io;
use regex::Regex;
use crate::parse_token::{ParseToken, LexType};

#[derive(Clone)]
pub(crate) struct RegexRecord {
    pub(crate) re_string: String,
    pub(crate) re: Regex,
}

fn insert_regex_record(regexes: &mut BTreeMap<LexType, RegexRecord>, tt: LexType, re_string: &str) -> () {
    regexes.insert(
        tt,
        RegexRecord {
            re_string: re_string.to_string(),
            re: Regex::new(re_string).unwrap(),
        },
    );
}

pub(crate) fn lexing_regexes() -> (BTreeMap<LexType, RegexRecord>, Regex) {
    let mut regexes = BTreeMap::new();
    for re_spec in vec![
        (LexType::Attribute, r#"(\|?[ \t]*[A-Za-z0-9\-]+="[^"]*")"#),
        (LexType::DefaultAttribute, r"(\|[ \t]*[^|\\]*)"),
        (LexType::EndMilestoneTag, r"(\\[*])"),
        (LexType::EndTag, r"(\\[+]?[a-z1-9\-]+[*])"),
        (LexType::StartTag, r"(\\[+]?[a-z1-9\-]+)"),
        (LexType::BareSlash, r"(\\)"),
        (LexType::Eol, r"([ \t]*[\r\n]+[ \t]*)"),
        (LexType::NoBreakSpace, r"~"),
        (LexType::SoftLineBreak, r"//"),
        (LexType::SpaceCharacters, r"(\p{Zs}{1,1024})"),
        (LexType::NonSpaceCharacters, r"([^\p{Zs}]{1,1024})"),
    ] {
        insert_regex_record(
            &mut regexes,
            re_spec.0,
            re_spec.1,
        );
    }

    let main_regex_str: String = regexes.iter()
        .map(|(_, v)| v.re_string.clone())
        .collect::<Vec<_>>()
        .join("|");
    let main_regex = Regex::new(&main_regex_str).unwrap();
    (regexes, main_regex)
}

pub fn lex_sfm(filename: &str) -> io::Result<Vec<ParseToken>> {
    let (lexing, main_regex) = lexing_regexes();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lexed: Vec<ParseToken> = Vec::new();
    for line in reader.lines() {
        for general_match in main_regex.find_iter(line?.as_str()) {
            for (tt, re_struct) in &lexing {
                let tt2 = tt.clone();
                if re_struct.re.is_match_at(general_match.as_str(), 0) {
                    lexed.push(ParseToken::Lexed {lex_type: tt2, matched: general_match.as_str().to_string()});
                    break;
                }
            }
        }
        lexed.push(ParseToken::Lexed {lex_type: LexType::Eol, matched: "\n".to_string()});
    };
    Ok(lexed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexing_regexes() {
        assert!(lexing_regexes().0.contains_key(&LexType::StartTag));
    }
    #[test]
    fn test_lex_sfm() {
        let first_lexed_token = &lex_sfm("test_data/usfm/hello.usfm").unwrap()[0];
        let first_lexed_data = match first_lexed_token {
            ParseToken::Lexed{lex_type, matched} => (lex_type, matched),
            _ => panic!("Unexpected token {}", first_lexed_token),
        };
        assert_eq!(*first_lexed_data.0, LexType::StartTag);
        assert_eq!(first_lexed_data.1, "\\id");
    }

}