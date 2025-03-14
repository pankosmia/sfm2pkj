use std::fs::File;
use std::io;
use std::io::BufRead;
use std::fmt;
mod lexing;
use crate::lexing::{lexing_regexes, TokenType};

pub struct LexedToken {
    token_type: TokenType,
    matched: String,
}

impl fmt::Display for LexedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[TokenType::{:?}: \"{}\"]", self.token_type, self.matched)
    }
}

pub fn tokenize_sfm(filename: &str) -> io::Result<Vec<LexedToken>> {
    let lexing = lexing_regexes();
    let main_regex = &lexing.get(&TokenType::Main).unwrap().re;
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut tokens: Vec<LexedToken> = Vec::new();
    for line in reader.lines() {
        for general_match in main_regex.find_iter(line?.as_str()) {
            for (tt, re_struct) in &lexing {
                let tt2 = tt.clone();
                if re_struct.re.is_match_at(general_match.as_str(), 0) {
                    tokens.push(LexedToken {token_type: tt2, matched: general_match.as_str().to_string()});
                    break;
                }
            }
        }
        tokens.push(LexedToken {token_type: TokenType::Eol, matched: "\n".to_string()});
    };
    Ok(tokens)
}

pub fn show_tokenized_sfm(filename: &str) -> () {
    for token in tokenize_sfm(filename).unwrap() {
        println!("{}", token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexing_regexes() {
        assert!(lexing_regexes().contains_key(&TokenType::Main));
    }
    #[test]
    fn test_tokenize_sfm() {
        assert_eq!(tokenize_sfm("test_data/usfm/hello.usfm").unwrap()[0].token_type, TokenType::StartTag);
        assert_eq!(tokenize_sfm("test_data/usfm/hello.usfm").unwrap()[0].matched, "\\id");
    }

    #[test]
    fn test_show_tokenized_sfm() {
        assert_eq!(show_tokenized_sfm("test_data/usfm/hello.usfm"), ());
    }
}