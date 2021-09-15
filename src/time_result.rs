use std::fmt;

pub type TimeResult<T> = Result<T, TimeError>;

pub enum TimeError {
    ParseInputError { input: String },
    ParseNumError { text: String },
}

impl fmt::Debug for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Error] {}", match self {
            TimeError::ParseInputError { input } =>
                format!("Failed to parse time from string \"{}\"", input),
            TimeError::ParseNumError { text } =>
                format!("Failed to parse string to number \"{}\"", text),
        })
    }
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
