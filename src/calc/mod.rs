use crate::time::Time;

pub fn add(a: Time, b: Time) -> Time {
    // Time::default()
    //     .with_days(a.get_days() + b.get_days())
    //     .with_hours(a.get_hours() + b.get_hours())
    //     .with_minutes(a.get_minutes() + b.get_minutes())
    //     .with_seconds(a.get_seconds() + b.get_seconds())
    //     .with_milliseconds(a.get_milliseconds() + b.get_milliseconds())

    let ms = a.as_milliseconds() + b.as_milliseconds();
    let res = TimeUnsafe::from_milliseconds(ms);
    Time::default()
        .with_days(res.days)
        .with_hours(res.hours)
        .with_minutes(res.minutes)
        .with_seconds(res.seconds)
        .with_milliseconds(res.milliseconds)
}

pub fn sub(a: Time, b: Time) -> Time {
    let (ms, is_positive) = {
        let ms_a = a.as_milliseconds();
        let ms_b = b.as_milliseconds();
        ms_a.checked_sub(ms_b)
            .map(|ms| (ms, true))
            .unwrap_or_else(|| (ms_b - ms_a, false))
    };
    let res = TimeUnsafe::from_milliseconds(ms);
    Time::default()
        .with_days(res.days)
        .with_hours(res.hours)
        .with_minutes(res.minutes)
        .with_seconds(res.seconds)
        .with_milliseconds(res.milliseconds)
        .with_is_positive(is_positive)
}

#[derive(Default)]
struct TimeUnsafe {
    pub days:         u32,
    pub hours:        u32,
    pub minutes:      u32,
    pub seconds:      u32,
    pub milliseconds: u32,
}

impl TimeUnsafe {
    pub fn from_milliseconds(mut milliseconds: u128) -> Self {
        let days = milliseconds / 1000 / 60 / 60 / 24;
        milliseconds -= days * 24 * 60 * 60 * 1000;
        let hours = milliseconds / 1000 / 60 / 60;
        milliseconds -= hours * 60 * 60 * 1000;
        let minutes = milliseconds / 1000 / 60;
        milliseconds -= minutes * 60 * 1000;
        let seconds = milliseconds / 1000;
        milliseconds -= seconds * 1000;
        // TODO safe conversions from u128 -> u32
        Self {
            days:         days as u32,
            hours:        hours as u32,
            minutes:      minutes as u32,
            seconds:      seconds as u32,
            milliseconds: milliseconds as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_times() {
        let a = Time::new("1 02:20:40.500").unwrap();
        let b = Time::new("2 23:50:30.655").unwrap();
        let expected = Time::new("4 02:11:11.155").unwrap();
        let actual = add(a, b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_subs_times() {
        let a = Time::new("2 23:50:30.655").unwrap();
        let b = Time::new("1 02:20:40.500").unwrap();
        let expected = Time::new("1 21:29:50.155").unwrap();
        let actual = sub(a, b);
        assert_eq!(expected, actual);
    }
}
