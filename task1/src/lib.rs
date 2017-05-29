
use std::fmt;
use std::fmt::Formatter;
use std::error::Error;

#[derive(Debug)]
pub enum Command {
    RETRIEVE,
    PUBLISH,
}

#[derive(Debug)]
pub struct Package {
    pub c_type: Command,
    pub message: String,
}

#[derive(Debug)]
pub enum ParseError {
    TooShortMessageError,
    RetrieveSyntaxError,
    NoPatternDetectedError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParseError::TooShortMessageError => write!(f, "Input message is too short."),
            ParseError::RetrieveSyntaxError => write!(f, "Wrong RETRIEVE syntax."),
            ParseError::NoPatternDetectedError => write!(f, "No pattern detected."),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::TooShortMessageError => "Input message is too short.",
            ParseError::RetrieveSyntaxError => "Wrong RETRIEVE syntax.",
            ParseError::NoPatternDetectedError => "No pattern detected.",
        }
    }
}

pub fn parse(message: &str) -> Result<Package, ParseError> {
    let mut msg: String = message.to_string();

    if msg.len() < 8 {
        Err(ParseError::TooShortMessageError)
    } else {
        let path = std::env::current_exe()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        msg = msg.replace(&(path + " "), "");

        if msg.starts_with("RETRIEVE\n") {
            Ok(Package {
                   c_type: Command::RETRIEVE,
                   message: "".to_string(),
               })
        } else if msg.starts_with("RETRIEVE ") {
            Err(ParseError::RetrieveSyntaxError)
        } else if msg.starts_with("PUBLISH ") {
            msg = msg.replacen("PUBLISH ", "", 1);
            msg.pop().unwrap();
            Ok(Package {
                   c_type: Command::PUBLISH,
                   message: msg.to_string(),
               })
        } else {
            Err(ParseError::NoPatternDetectedError)
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_package_message_publish() {
        let input_string = "PUBLISH Hallo das ist ein publish\n";
        let result = parse(input_string);
        assert_eq!("Hallo das ist ein publish", result.unwrap().message);
    }

    #[test]
    pub fn test_package_message_retrieve() {
        let input_string = "RETRIEVE\n";
        let result = parse(input_string);
        assert_eq!("", result.unwrap().message);
    }

    #[test]
    pub fn test_package_c_type_publish() {
        let input_string = "PUBLISH Hallo das ist ein publish\n";
        let result = parse(input_string);
        assert_eq!(Command::PUBLISH as u8, result.unwrap().c_type as u8);
    }

    #[test]
    pub fn test_package_c_type_retrieve() {
        let input_string = "RETRIEVE\n";
        let result = parse(input_string);
        assert_eq!(Command::RETRIEVE as u8, result.unwrap().c_type as u8);
    }

    #[test]
    pub fn test_parse_no_pattern_1() {
        let input_string = "Das ist ein test\n";
        let result = parse(input_string);
        assert_eq!("No pattern detected.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_no_pattern_2() {
        let input_string = "\n\n\n\n\n\n\n\n\n\n\n\n";
        let result = parse(input_string);
        assert_eq!("No pattern detected.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_no_pattern_3() {
        let input_string = "TEST PUBLISH something\n";
        let result = parse(input_string);
        assert_eq!("No pattern detected.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_no_pattern_4() {
        let input_string = "TEST PUBLISH RETRIEVE something\n";
        let result = parse(input_string);
        assert_eq!("No pattern detected.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_input_too_short_1() {
        let input_string = "PUBLIS\n";
        let result = parse(input_string);
        assert_eq!("Input message is too short.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_input_too_short_2() {
        let input_string = "PUBLISH";
        let result = parse(input_string);
        assert_eq!("Input message is too short.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_input_too_short_3() {
        let input_string = "";
        let result = parse(input_string);
        assert_eq!("Input message is too short.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_valid_input_1() {
        let input_string = "PUBLISH \n";
        assert_eq!("", parse(input_string).unwrap().message);
        assert_eq!(Command::PUBLISH as u8,
                   parse(input_string).unwrap().c_type as u8);
    }

    #[test]
    pub fn test_parse_valid_input_2() {
        let input_string = "RETRIEVE\n";
        assert_eq!("", parse(input_string).unwrap().message);
        assert_eq!(Command::RETRIEVE as u8,
                   parse(input_string).unwrap().c_type as u8);
    }

    #[test]
    pub fn test_parse_wrong_input_1() {
        let input_string = "PUBLISH\n";
        let result = parse(input_string);
        assert_eq!("No pattern detected.", result.err().unwrap());
    }

    #[test]
    pub fn test_parse_wrong_input_2() {
        let input_string = "RETRIEVE \n";
        let result = parse(input_string);
        assert_eq!("Wrong RETRIEVE syntax.", result.err().unwrap());
    }
}
