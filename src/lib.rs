mod lexing;
mod parse_token;
use crate::lexing::lex_sfm;

pub fn show_lexed_sfm(filename: &str) -> () {
    for lex in lex_sfm(filename).unwrap() {
        println!("{}", lex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_show_lexed_sfm() {
        for data_name in vec![
            "hello",
            "soft_line_break",
            "milestone_attributes",
            "milestone_attributes_default",
            "standalone_milestone"
        ] {
            println!("\n*** {} ***\n", data_name.to_ascii_uppercase());
            assert_eq!(show_lexed_sfm(format!("test_data/usfm/{}.usfm", data_name).as_str()), ());
        }
    }

}