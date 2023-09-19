pub fn panic_message() {
    panic!("Panicked!");
}

// -------------- 
// --- OPTION --- 
// -------------- 

pub fn optional_five() -> Option<u32> {
    Some(5)
}

// Prefer `expect` over `unwrap` for `Option` as you can add a custom error message
pub fn five() -> u32 {
    Some(5).expect("Panicking!")
}

pub fn optional_six() -> Option<u32> {
    None
}

// -------------- 
// --- RESULT --- 
// -------------- 

// Result<u32, ()> is semantically equivalent to Option<u32>
pub fn result_five() -> Result<u32, ()> {
    Ok(5)
}

pub fn parse_integer(num_str: &str) -> Result<i32, std::num::ParseIntError> {
    num_str.parse()
}

pub fn parse_integer_question_operator(num_str: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = num_str.parse()?;
    Ok(num)
}

pub fn double_number_match(num_str: &str) -> Result<i32, std::num::ParseIntError> {
    match parse_integer(num_str) {
        Ok(num) => Ok(num * 2),
        Err(err) => Err(err)
    }
}

pub fn double_number_map(num_str: &str) -> Result<i32, std::num::ParseIntError> {
    parse_integer(num_str).map(|x| x * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn assert_panicked() {
        assert_eq!(panic_message(), ());
    }

    #[test]
    fn get_some() {
        assert_eq!(optional_five(), Some(5));
    }

    #[test]
    fn get_five() {
        assert_eq!(five(), 5);
    }

    #[test]
    fn get_none() {
        assert_eq!(optional_six(), None);
    }

    #[test]
    fn get_result() {
        assert_eq!(result_five(), Ok(5));
    }

    #[test]
    fn unwrap_result() {
        assert_eq!(result_five().unwrap(), 5);
    }

    #[test]
    fn parse_five() {
        assert_eq!(parse_integer("5"), Ok(5));
    }

    #[test]
    fn double_five() {
        assert_eq!(double_number_match("5"), Ok(10));
    }

    #[test]
    fn double_six() {
        assert_eq!(double_number_map("6"), Ok(12));
    }
}
