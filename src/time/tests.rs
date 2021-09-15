#![cfg(test)]

use super::Time;

#[test]
fn it_overflows_hours() {
    let time = Time::default().with_hours(26);
    assert_eq!(1, time.get_days());
    assert_eq!(2, time.get_hours());
    assert_eq!(0, time.get_minutes());
    assert_eq!(0, time.get_seconds());
    assert_eq!(0, time.get_milliseconds());
}

#[test]
fn it_overflows_minutes() {
    let time = Time::default().with_minutes(1590);
    assert_eq!(1, time.get_days());
    assert_eq!(2, time.get_hours());
    assert_eq!(30, time.get_minutes());
    assert_eq!(0, time.get_seconds());
    assert_eq!(0, time.get_milliseconds());
}

#[test]
fn it_overflows_seconds() {
    let time = Time::default().with_seconds(95440);
    assert_eq!(1, time.get_days());
    assert_eq!(2, time.get_hours());
    assert_eq!(30, time.get_minutes());
    assert_eq!(40, time.get_seconds());
    assert_eq!(0, time.get_milliseconds());
}

#[test]
fn it_overflows_milliseconds() {
    let time = Time::default().with_milliseconds(95440500);
    assert_eq!(1, time.get_days());
    assert_eq!(2, time.get_hours());
    assert_eq!(30, time.get_minutes());
    assert_eq!(40, time.get_seconds());
    assert_eq!(500, time.get_milliseconds());
}

#[test]
fn it_formats_to_string() {
    let time = Time::default()
        .with_days(1)
        .with_hours(2)
        .with_minutes(3)
        .with_seconds(4)
        .with_milliseconds(50);
    let expected = String::from("1 02:03:04.050");
    let actual = time.to_string();
    assert_eq!(expected, actual);
}
