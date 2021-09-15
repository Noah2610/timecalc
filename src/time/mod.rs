mod tests;

use crate::parse::parse_time;
use crate::time_result::TimeResult;

#[derive(PartialEq, Eq, Debug)]
pub struct Time {
    days:         u32,
    hours:        u32,
    minutes:      u32,
    seconds:      u32,
    milliseconds: u32,
    is_positive:  bool,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            days:         0,
            hours:        0,
            minutes:      0,
            seconds:      0,
            milliseconds: 0,
            is_positive:  true,
        }
    }
}

impl Time {
    pub fn new<S: ToString>(s: S) -> TimeResult<Self> {
        parse_time(s)
    }

    pub fn as_milliseconds(&self) -> u128 {
        (self.days as u128 * 24 * 60 * 60 * 1000)
            + (self.hours as u128 * 60 * 60 * 1000)
            + (self.minutes as u128 * 60 * 1000)
            + (self.seconds as u128 * 1000)
            + (self.milliseconds as u128)
    }

    pub fn get_days(&self) -> u32 {
        self.days
    }
    pub fn set_days(&mut self, days: u32) {
        self.days = days;
    }
    pub fn with_days(mut self, days: u32) -> Self {
        self.set_days(days);
        self
    }

    pub fn get_hours(&self) -> u32 {
        self.hours
    }
    pub fn set_hours(&mut self, hours: u32) {
        self.hours = hours;
        self.fix_hours();
    }
    pub fn with_hours(mut self, hours: u32) -> Self {
        self.set_hours(hours);
        self
    }

    pub fn get_minutes(&self) -> u32 {
        self.minutes
    }
    pub fn set_minutes(&mut self, minutes: u32) {
        self.minutes = minutes;
        self.fix_minutes();
    }
    pub fn with_minutes(mut self, minutes: u32) -> Self {
        self.set_minutes(minutes);
        self
    }

    pub fn get_seconds(&self) -> u32 {
        self.seconds
    }
    pub fn set_seconds(&mut self, seconds: u32) {
        self.seconds = seconds;
        self.fix_seconds();
    }
    pub fn with_seconds(mut self, seconds: u32) -> Self {
        self.set_seconds(seconds);
        self
    }

    pub fn get_milliseconds(&self) -> u32 {
        self.milliseconds
    }
    pub fn set_milliseconds(&mut self, milliseconds: u32) {
        self.milliseconds = milliseconds;
        self.fix_milliseconds();
    }
    pub fn with_milliseconds(mut self, milliseconds: u32) -> Self {
        self.set_milliseconds(milliseconds);
        self
    }

    pub fn get_is_positive(&self) -> bool {
        self.is_positive
    }
    pub fn set_is_positive(&mut self, is_positive: bool) {
        self.is_positive = is_positive;
    }
    pub fn with_is_positive(mut self, is_positive: bool) -> Self {
        self.set_is_positive(is_positive);
        self
    }

    fn fix_hours(&mut self) {
        self.days += self.hours / 24;
        self.hours %= 24;
    }

    fn fix_minutes(&mut self) {
        self.hours += self.minutes / 60;
        self.minutes %= 60;
        self.fix_hours();
    }

    fn fix_seconds(&mut self) {
        self.minutes += self.seconds / 60;
        self.seconds %= 60;
        self.fix_minutes();
    }

    fn fix_milliseconds(&mut self) {
        self.seconds += self.milliseconds / 1000;
        self.milliseconds %= 1000;
        self.fix_seconds();
    }
}
