use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u32,
    height: u32,
    nationality: Nationality,
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
    Ok(p)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use std::{fs, path::PathBuf};

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn read_json_fixture_file(path: &str) -> Result<String> {
        // For info on CARGO_MANIFEST_DIR:
        // https://doc.rust-lang.org/cargo/reference/environment-variables.html
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push(path);

        let input = fs::read_to_string(file).expect(path);
        Ok(input)
    }

    #[test]
    fn read_json_to_struct() {
        let person_json_string = read_json_fixture_file("fixtures/person.json").unwrap();
        let p = read_json_person(person_json_string).unwrap();
        assert_eq!(p.name, "Benjamin");
        assert_eq!(p.age, 26);
        assert_eq!(p.height, 188);
        assert_eq!(p.nationality, Nationality::Switzerland);
    }

    #[test]
    fn read_json() {
        let complex_json_string = read_json_fixture_file("fixtures/complex.json").unwrap();
        let json: serde_json::Value =
            serde_json::from_str(&complex_json_string).expect("JSON was not well-formatted");
        assert_eq!(json["name"], json!("John"));
        assert_eq!(json["siblings"], json!(["Brad", "Chad"]));
        assert_eq!(
            json["address"],
            json!(
            {
                "street": "Vera",
                "number":1350,
                "country": "Argentina",
                "floor": None::<u32>,
                "current": true,
            })
        );
        // To see output run: `cargo test -- --nocapture`
        println!("{:?}", json);
    }
}
