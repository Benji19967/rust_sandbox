#[cfg(test)]
mod tests {
    use regex::Regex;

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
    // find: get the byte offsets of the first match and the actual substring that matched
    //
    #[test]
    fn find1() {
        let re = Regex::new(r"Ben").unwrap();
        let mat = re.find("Ben Ben").unwrap();
        assert_eq!(mat.range(), 0..3);
        assert_eq!(mat.as_str(), "Ben");
    }
    #[test]
    fn find2() {
        let re = Regex::new(r"Beno").unwrap();
        let mat = re.find("Ben Ben");
        assert_eq!(mat, None);
    }
    
    //
    // find_iter: find all successive non-overlapping matches
    //
    #[test]
    fn find_iter1() {
        let re = Regex::new(r"Ben").unwrap();
        let matches_range: Vec<_> = re.find_iter("Ben Ben").map(|x| x.range()).collect();
        let matches_str: Vec<_> = re.find_iter("Ben Ben").map(|x| x.as_str()).collect();
        assert_eq!(matches_range, vec![0..3, 4..7]);
        assert_eq!(matches_str, vec!["Ben", "Ben"]);
    }
    #[test]
    fn find_iter2() {
        let re = Regex::new(r"Beno").unwrap();
        let matches: Vec<_> = re.find_iter("Ben Ben").collect();
        assert_eq!(matches, vec![]);
    }
    
    //
    // captures: find first match and return that match as well as all capture groups
    //
    #[test]
    fn captures1() {
        // [^'] matches any char that is not ' (since ^ negates)
        let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
        let captures = re.captures("The name is: 'Ben' (1996).").unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "'Ben' (1996)");
        assert_eq!(captures.get(1).unwrap().as_str(), "Ben");
        assert_eq!(captures.get(2).unwrap().as_str(), "1996");
    }
    #[test]
    fn captures2() {
        // Named captures: add ?<a_name> in the group
        let re = Regex::new(r"'(?<username>[^']+)'\s+\((?<year>\d{4})\)").unwrap();
        let captures = re.captures("The name is: 'Ben' (1996).").unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "'Ben' (1996)");
        assert_eq!(captures.name("username").unwrap().as_str(), "Ben");
        assert_eq!(captures.name("year").unwrap().as_str(), "1996");
    }
}
