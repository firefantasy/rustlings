// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::error;
use std::str::FromStr;
use std::fmt;
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 5. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object

#[derive(Debug)]
struct AgeNotNumber;
impl error::Error for AgeNotNumber {}

impl fmt::Display for AgeNotNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "myerr")
    }
}

#[derive(Debug)]
struct LengthErr;
impl error::Error for LengthErr {}

impl fmt::Display for LengthErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length must be two")
    }
}

#[derive(Debug)]
struct NameIsEmpty;
impl error::Error for NameIsEmpty {}

impl fmt::Display for NameIsEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length must be two")
    }
}

impl FromStr for Person {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let arr: Vec<&str> = s.split(',').collect();
        let mut person = Person{
            name: "".to_owned(),
            age: 0
        };
        match arr.len() {
            2 => {  
                let name = arr[0].to_string();
                if name == "" { return Err(Box::new(NameIsEmpty));} 
                let age: usize = match arr[1].parse::<usize>() {
                    Ok(i) => {
                        i
                    },
                    Err(_) => {
                        return Err(Box::new(AgeNotNumber));
                    }
                };
                person.name = name;
                person.age = age;
                Ok(person)
            },
            _ => {return Err(Box::new(LengthErr));}
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!("John,".parse::<Person>().is_err());
    }

    #[test]
    fn invalid_age() {
        assert!("John,twenty".parse::<Person>().is_err());
    }

    #[test]
    fn missing_comma_and_age() {
        assert!("John".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name() {
        assert!(",1".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_age() {
        assert!(",".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(",one".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma() {
        assert!("John,32,".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert!("John,32,man".parse::<Person>().is_err());
    }
}
