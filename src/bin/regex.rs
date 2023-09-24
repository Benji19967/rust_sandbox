use regex::Regex;

// https://docs.rs/regex/latest/regex/

fn main() {
    // Extract a phone number
    //let phone1 = "This is a phone number: +44 999 832 7777"
    //let phone2 = "This is a phone number: +449998327777"
    //let phone3 = "This is a phone number: +(44)-999-832-7777"

}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // is_match: does a pattern match?
    //

    #[test]
    fn is_match1() {
        let re = Regex::new(r"Ben").unwrap();
        assert!(re.is_match("Ben"));
    }
    #[test]
    fn is_match2() {
        let re = Regex::new(r"Beno").unwrap();
        assert!(!re.is_match("Ben"));
    }

    //
    // find: get the byte offsets of the match and the actual substring that matched
    //
    
    #[test]
    fn find1() {
        let re = Regex::new(r"Ben").unwrap();
        let mat = re.find("Ben").unwrap();
        assert_eq!(mat.range(), 0..3);
        assert_eq!(mat.as_str(), "Ben");
    }
    #[test]
    fn find2() {
        let re = Regex::new(r"Beno").unwrap();
        let mat = re.find("Ben");
        assert_eq!(mat, None);
    }
}
