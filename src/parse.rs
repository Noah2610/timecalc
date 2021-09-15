use crate::time::Time;
use crate::time_result::{TimeError, TimeResult};

pub fn parse_time<S: ToString>(input: S) -> TimeResult<Time> {
    let re = regex::Regex::new(r"(?P<num>\d{1,2}):?").unwrap();

    let input = input.to_string();

    let mut nums: Vec<u32> = Vec::new();

    // for cap in caps.iter().skip(1) {
    for caps in re.captures_iter(input.as_str()) {
        // let caps = re.captures(input.as_str()).ok_or_else(|| {
        //     TimeError::ParseInputError {
        //         input: input.to_string(),
        //     }
        // })?;

        let num = parse_num(&caps["num"])?;
        nums.push(num);
    }

    match (nums.get(0), nums.get(1), nums.get(2)) {
        (None, None, None) => Err(TimeError::ParseInputError {
            input: input.to_string(),
        }),
        (Some(&hours), None, None) => Ok(Time {
            hours,
            ..Default::default()
        }),
        (Some(&hours), Some(&minutes), None) => Ok(Time {
            hours,
            minutes,
            ..Default::default()
        }),
        (Some(&hours), Some(&minutes), Some(&seconds)) => Ok(Time {
            hours,
            minutes,
            seconds,
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
    #[ignore]
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
    #[should_panic]
    fn it_doesnt_parse_invalid_input() {
        parse_time("nope").unwrap();
    }
}
