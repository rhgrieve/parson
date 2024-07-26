use std::process;

pub fn parse_opt(opt: String) -> (String, String) {
    let raw_opt: Vec<&str> = opt.trim_matches('-').split('=').collect();
    let opt_name = raw_opt[0];
    let mut opt_value = "";
    if raw_opt.len() > 1 {
        opt_value = raw_opt[1];
    }
    (opt_name.to_string(), opt_value.to_string())
}

pub enum ErrorType<'a> {
    InvalidOption(&'a str),
    ExpectingValue(&'a str),
    UnexpectedValue(&'a str),
    ShortExpectingValue(&'a str),
}

pub fn error(error: ErrorType) {
    match error {
        ErrorType::InvalidOption(s) => {
            eprintln!("Invalid option `{s}`");
        }
        ErrorType::ExpectingValue(s) => {
            eprintln!("Option `{s}` expects a value");
        }
        ErrorType::UnexpectedValue(s) => {
            eprintln!("Option `{s}` does not expect a value, but one was provided");
        }
        ErrorType::ShortExpectingValue(s) => {
            eprintln!("Short option `{s}` expects a value and must be passed separately");
        }
    }
    process::exit(2)
}
