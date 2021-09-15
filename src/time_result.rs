use std::fmt;

pub type TimeResult<T> = Result<T, TimeError>;

pub enum TimeError {
    ParseInputError { input: String },
    ParseNumError { text: String, name: Option<String> },
}

impl fmt::Debug for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Error] {}", match self {
            TimeError::ParseInputError { input } =>
                format!("Failed to parse time from string \"{}\"", input),
            TimeError::ParseNumError { text, name } => format!(
                "Failed to parse {name} to number \"{text}\"",
                text = text,
                name = name.as_ref().map(String::as_str).unwrap_or("string"),
            ),
        })
    }
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
