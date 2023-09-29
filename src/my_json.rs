use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u32,
    height: u32,
    nationality: Nationality
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Nationality {
    Switzerland,
    Canada,
    UnitedStatesOfAmerica,
    Argentina,
}


pub fn read_json_person(data: String) -> Result<Person> {
    let p: Person = serde_json::from_str(&data)?;
    println!("{:?}", p);
    Ok(p)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn read_json_fixture_file() -> Result<String> {
        // For info on CARGO_MANIFEST_DIR:
        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/person.json");
        let file = PathBuf::from(path);

        let input = fs::read_to_string(file).expect(path);
        Ok(input)
    }

    #[test]
    fn read_json() {
        let person_json_string =  read_json_fixture_file().unwrap();
        let p = read_json_person(person_json_string).unwrap();
        assert_eq!(p.name, "Benjamin");
        assert_eq!(p.age, 26);
        assert_eq!(p.height, 188);
        assert_eq!(p.nationality, Nationality::Switzerland);
    }
}
