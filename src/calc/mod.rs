use crate::time::Time;

pub fn add(a: Time, b: Time) -> Time {
    Time::default()
        .with_days(a.get_days() + b.get_days())
        .with_hours(a.get_hours() + b.get_hours())
        .with_minutes(a.get_minutes() + b.get_minutes())
        .with_seconds(a.get_seconds() + b.get_seconds())
        .with_milliseconds(a.get_milliseconds() + b.get_milliseconds())
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

    // #[test]
    // fn it_subs_times() {
    //     let a = Time::new("2 23:50:30.655").unwrap();
    //     let b = Time::new("1 02:20:40.500").unwrap();
    //     let expected = Time::new("1 21:11:11.155").unwrap();
    //     let actual = sub(a, b);
    //     assert_eq!(expected, actual);
    // }
}
