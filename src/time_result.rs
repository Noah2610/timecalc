use std::fmt;

pub type TimeResult<T> = Result<T, TimeError>;

pub enum TimeError {
    ParseError { input: String },
}

impl fmt::Debug for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Error] {}", match self {
            TimeError::ParseError { input } =>
                format!("Failed to parse time from string \"{}\"", input),
        })
    }
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
