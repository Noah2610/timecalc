use crate::time::Time;
use crate::time_result::{TimeError, TimeResult};

pub fn parse_time<S: ToString>(input: S) -> TimeResult<Time> {
    let re = regex::Regex::new(
        r"(?:(?P<days>\d+) )?(?P<num>\d{1,2}):?(?:\.(?P<millis>\d{1,3})$\s*)?",
    )
    .unwrap();

    let input = input.to_string();

    let mut nums: Vec<u32> = Vec::new();
    let mut days: Option<u32> = None;
    let mut millis: Option<u32> = None;

    // for cap in caps.iter().skip(1) {
    for caps in re.captures_iter(input.as_str()) {
        // let caps = re.captures(input.as_str()).ok_or_else(|| {
        //     TimeError::ParseInputError {
        //         input: input.to_string(),
        //     }
        // })?;

        let num = parse_num(&caps["num"])?;
        nums.push(num);

        if let Some(cap_days) = caps.name("days") {
            days = Some(parse_num(cap_days.as_str())?);
        }
        if let Some(cap_millis) = caps.name("millis") {
            millis = Some(parse_num(cap_millis.as_str())?);
        }
    }

    match (nums.get(0), nums.get(1), nums.get(2)) {
        (None, None, None) => Err(TimeError::ParseInputError {
            input: input.to_string(),
        }),
        (Some(&hours), None, None) => Ok(Time {
            days: days.unwrap_or(0),
            hours,
            milliseconds: millis.unwrap_or(0),
            ..Default::default()
        }),
        (Some(&hours), Some(&minutes), None) => Ok(Time {
            days: days.unwrap_or(0),
            hours,
            minutes,
            milliseconds: millis.unwrap_or(0),
            ..Default::default()
        }),
        (Some(&hours), Some(&minutes), Some(&seconds)) => Ok(Time {
            days: days.unwrap_or(0),
            hours,
            minutes,
            seconds,
            milliseconds: millis.unwrap_or(0),
            ..Default::default()
        }),
        _ => unreachable!(),
    }
}

fn parse_num(s: &str) -> TimeResult<u32> {
    s.parse::<u32>().or_else(|_| {
        Err(TimeError::ParseNumError {
            text: s.to_string(),
        })
    })
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

    #[test]
    fn it_parses_hours_and_minutes_short() {
        let expected = Time {
            hours: 1,
            minutes: 23,
            ..Default::default()
        };
        let actual = parse_time("0123").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_hours_minutes_and_seconds_short() {
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
            ..Default::default()
        };
        let actual = parse_time("012345").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_hours_minutes_seconds_and_milliseconds_short() {
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
            milliseconds: 678,
            ..Default::default()
        };
        let actual = parse_time("012345.678").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_days_with_colon() {
        let expected = Time {
            days: 1,
            hours: 2,
            minutes: 34,
            seconds: 56,
            milliseconds: 789,
            ..Default::default()
        };
        let actual = parse_time("1 02:34:56.789").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parses_days_short() {
        let expected = Time {
            days: 1,
            hours: 2,
            minutes: 34,
            seconds: 56,
            milliseconds: 789,
            ..Default::default()
        };
        let actual = parse_time("01 023456.789").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn it_doesnt_parse_invalid_input() {
        parse_time("nope").unwrap();
    }
}
