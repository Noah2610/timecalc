use crate::parse::parse_time;
use crate::time_result::TimeResult;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Time {
    pub days:         u32,
    pub hours:        u32,
    pub minutes:      u32,
    pub seconds:      u32,
    pub milliseconds: u32,
}

impl Time {
    pub fn new<S: ToString>(s: S) -> TimeResult<Self> {
        parse_time(s)
    }
}
