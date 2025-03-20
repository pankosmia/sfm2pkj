use crate::parse_token::{LexType, ParseToken, StartMilestoneType};

pub fn handle_milestones(lexed: &Vec<ParseToken>) -> Vec<ParseToken> {
    let mut ret = Vec::new();
    let mut index = 0;
    let mut current_start_token = None;
    while index < lexed.len() {
        let lexed_token = &lexed[index];
        match lexed_token {
            ParseToken::Lexed { lex_type, matched } => {
                match lex_type {
                    LexType::StartTag => { // Grab the value for later
                        current_start_token = Some(lexed_token);
                        index += 1;
                    }
                    LexType::EndMilestoneTag => { // Make an empty milestone
                        match current_start_token {
                            Some(start_token) => {
                                match start_token {
                                    ParseToken::Lexed { lex_type: _, matched } => {
                                        ret.push(ParseToken::StartMilestone { milestone_type: StartMilestoneType::Empty, tag: matched.clone() });
                                        current_start_token = None;
                                    },
                                    _ => {}
                                }
                            }
                            None => {}
                        };
                        index += 1;
                    }
                    _ => { // Clear start token and continue
                        println!("Other");
                        current_start_token = None;
                        ret.push(lexed_token.clone());
                        index += 1;
                    }
                }
            }
            _ => panic!("Unexpected token {}", lexed_token)
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::parse_token::{LexType, StartMilestoneType};
    use super::*;

    #[test]
    fn test_handle_milestones() {
        let mut test_vec = Vec::new();
        test_vec.push(ParseToken::Lexed { lex_type: LexType::StartTag, matched: "\\ts".to_string() });
        test_vec.push(ParseToken::Lexed { lex_type: LexType::EndMilestoneTag, matched: "\\*".to_string() });
        let milestoned = handle_milestones(&test_vec);
        assert_eq!(milestoned.len(), 1);
        println!("{}", milestoned[0]);
    }
}