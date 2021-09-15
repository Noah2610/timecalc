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

        let num = parse_num(&caps["num"], None)?;
        nums.push(num);

        if let Some(cap_days) = caps.name("days") {
            days = Some(parse_num(cap_days.as_str(), Some("days"))?);
        }
        if let Some(cap_millis) = caps.name("millis") {
            millis =
                Some(parse_num(cap_millis.as_str(), Some("milliseconds"))?);
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

fn parse_num(s: &str, name: Option<&str>) -> TimeResult<u32> {
    s.parse::<u32>().or_else(|_| {
        Err(TimeError::ParseNumError {
            text: s.to_string(),
            name: name.map(ToString::to_string),
        })
    })
}
