use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum LexType {
    Attribute,
    DefaultAttribute,
    EndMilestoneTag,
    EndTag,
    StartTag,
    BareSlash,
    Eol,
    NoBreakSpace,
    SoftLineBreak,
    SpaceCharacters,
    NonSpaceCharacters,
}
pub struct Lexed {
    pub lex_type: LexType,
    pub matched: String,
}

impl fmt::Display for Lexed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[TokenType::{:?}: \"{}\"]", self.lex_type, self.matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexed_display() {
        assert!(format!("{}", Lexed {lex_type: LexType::NonSpaceCharacters, matched: "foo".to_string()}).len() > 0);
    }

}