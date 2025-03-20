use std::fmt;
use std::cmp::PartialEq;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Copy)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Copy)]
pub enum StartMilestoneType {
    Explicit,
    Empty
}

#[derive(Clone)]
pub enum ParseToken {
    Lexed {lex_type: LexType, matched: String},
    StartMilestone {milestone_type: StartMilestoneType, tag: String}
}

impl fmt::Display for ParseToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseToken::Lexed{lex_type, matched} => write!(f, "[Lexed {:?}: \"{}\"]", lex_type, matched),
            ParseToken::StartMilestone{milestone_type, tag} => write!(f, "[StartMS {:?}: \"{}\"]", milestone_type, tag)
        }
    }
}

impl PartialEq for ParseToken {
    fn eq(&self, other: &Self) -> bool {
            match self {
                ParseToken::Lexed{lex_type, matched} => {
                    let lt = lex_type;
                    let m = matched;
                    match other {
                        ParseToken::Lexed{lex_type, matched} => m == matched && lt == lex_type,
                        _ => false
                    }
                },
                ParseToken::StartMilestone{milestone_type, tag} => {
                    let mt = milestone_type;
                    let t = tag;
                    match other {
                        ParseToken::StartMilestone{milestone_type, tag} => mt == milestone_type && t == tag,
                        _ => false
                    }
                }
            }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexed_display() {
        assert!(format!("{}", ParseToken::Lexed {lex_type: LexType::NonSpaceCharacters, matched: "foo".to_string()}).len() > 0);
    }

}