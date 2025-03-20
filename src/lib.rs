mod lexing;
mod parse_token;
mod milestones;

use crate::lexing::lex_sfm;
use crate::milestones::handle_milestones;

pub fn sfm2pkj(filename: &str) -> () {
    let lexed = lex_sfm(filename).unwrap();
    println!("\n* Lexed *");
    for lex in &lexed {
        println!("{}", lex);
    }
    let milestoned = handle_milestones(&lexed);
    println!("\n* Milestoned *");
    for token in &milestoned {
        println!("{}", token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sfm2pkj() {
        for data_name in vec![
            "hello",
            "soft_line_break",
            "milestone_attributes",
            "milestone_attributes_default",
            "standalone_milestone"
        ] {
            println!("\n*** {} ***\n", data_name.to_ascii_uppercase());
            assert_eq!(sfm2pkj(format!("test_data/usfm/{}.usfm", data_name).as_str()), ());
        }
    }

}