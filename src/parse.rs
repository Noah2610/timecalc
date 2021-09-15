use crate::time::Time;
use crate::time_result::TimeResult;

pub fn parse_time<S: ToString>(input: S) -> TimeResult<Time> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::parse_time;
    use crate::time::Time;

    #[test]
    fn it_parses_hours() {
        let expected = Time {
            hours: 3,
            ..Default::default()
        };
        let actual = parse_time("3").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_hours_and_minutes_with_colon() {
        let expected = Time {
            hours: 1,
            minutes: 23,
            ..Default::default()
        };
        let actual = parse_time("01:23").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_hours_minutes_and_seconds_with_colon() {
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
            ..Default::default()
        };
        let actual = parse_time("01:23:45").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_hours_minutes_seconds_and_milliseconds_with_colon() {
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
            milliseconds: 678,
            ..Default::default()
        };
        let actual = parse_time("01:23:45.678").unwrap();
        assert_eq!(expected, actual);
    }
}
