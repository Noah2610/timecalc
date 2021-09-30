use crate::time::Time;
use crate::time_result::{TimeError, TimeResult};
use regex::Regex;

pub fn parse_time<S: ToString>(input: S) -> TimeResult<Time> {
    let input = input.to_string();

    let has_colon = input.contains(":");

    let re = if has_colon {
        Regex::new(
            r"(?:(?P<days>\d+) )?(?P<num>\d+):?(?:\.(?P<millis>\d+)$\s*)?",
        )
    } else {
        Regex::new(
            r"(?:(?P<days>\d+) )?(?P<num>\d{1,2})(?:\.(?P<millis>\d+)$\s*)?",
        )
    }
    .unwrap();

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

    if nums.is_empty() && days.is_none() && millis.is_none() {
        return Err(TimeError::ParseInputError {
            input: input.to_string(),
        });
    }

    let mut time = Time::default();
    let mut iter = nums.into_iter();

    days.map(|days| time.set_days(days));
    iter.next().map(|hours| time.set_hours(hours));
    iter.next().map(|minutes| time.set_minutes(minutes));
    iter.next().map(|seconds| time.set_seconds(seconds));
    millis.map(|millis| time.set_milliseconds(millis));

    Ok(time)

    // iter.next()
    //     .map(|hours| {
    //         let time = time.with_hours(hours);
    //         iter.next()
    //             .map(|minutes| {
    //                 let time = time.with_minutes(minutes);
    //                 iter.next()
    //                     .map(|seconds| time.with_seconds(seconds))
    //                     .or_else(time)
    //             })
    //             .or_else(time)
    //     })
    //     .or_else(time);

    // if let Some(hours) = iter.next() {
    //     time.set_hours(hours);
    // }
    // if let Some(minutes) = iter.next() {
    //     time.set_minutes(minutes);
    // }
    // if let Some(seconds) = iter.next() {
    //     time.set_seconds(seconds);
    // }

    // match (nums.get(0), nums.get(1), nums.get(2)) {
    //     (None, None, None) => Err(TimeError::ParseInputError {
    //         input: input.to_string(),
    //     }),
    //     (Some(&hours), None, None) => Ok({
    //         let mut time = Time::default();
    //         if let Some(days) = days {
    //             time.set_days(days);
    //         }
    //         if let Some(milliseconds) = millis {
    //             time.set_milliseconds(milliseconds);
    //         }
    //         time.with_hours(hours)
    //     }),
    //     (Some(&hours), Some(&minutes), None) => Ok({
    //         let mut time = Time::default();
    //         if let Some(days) = days {
    //             time.set_days(days);
    //         }
    //         if let Some(milliseconds) = millis {
    //             time.set_milliseconds(milliseconds);
    //         }
    //         time.with_hours(hours).with_minutes(minutes)
    //     }),
    //     (Some(&hours), Some(&minutes), Some(&seconds)) => Ok({
    //         let mut time = Time::default();
    //         if let Some(days) = days {
    //             time.set_days(days);
    //         }
    //         if let Some(milliseconds) = millis {
    //             time.set_milliseconds(milliseconds);
    //         }
    //         time.with_hours(hours)
    //             .with_minutes(minutes)
    //             .with_seconds(seconds)
    //     }),
    //     _ => unreachable!(),
    // }
}

fn parse_num(s: &str, name: Option<&str>) -> TimeResult<u32> {
    s.parse::<u32>().or_else(|_| {
        Err(TimeError::ParseNumError {
            text: s.to_string(),
            name: name.map(ToString::to_string),
        })
    })
}
