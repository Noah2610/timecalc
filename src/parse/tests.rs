#![cfg(test)]

use super::parse_time;
use crate::time::Time;

#[test]
fn it_parses_hours() {
    let expected = Time::default().with_hours(3);
    let actual = parse_time("3").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_hours_and_minutes_with_colon() {
    let expected = Time::default().with_hours(1).with_minutes(23);
    let actual = parse_time("01:23").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_hours_minutes_and_seconds_with_colon() {
    let expected = Time::default()
        .with_hours(1)
        .with_minutes(23)
        .with_seconds(45);
    let actual = parse_time("01:23:45").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_hours_minutes_seconds_and_milliseconds_with_colon() {
    let expected = Time::default()
        .with_hours(1)
        .with_minutes(23)
        .with_seconds(45)
        .with_milliseconds(678);
    let actual = parse_time("01:23:45.678").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_hours_and_minutes_short() {
    let expected = Time::default().with_hours(1).with_minutes(23);
    let actual = parse_time("0123").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_hours_minutes_and_seconds_short() {
    let expected = Time::default()
        .with_hours(1)
        .with_minutes(23)
        .with_seconds(45);
    let actual = parse_time("012345").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_hours_minutes_seconds_and_milliseconds_short() {
    let expected = Time::default()
        .with_hours(1)
        .with_minutes(23)
        .with_seconds(45)
        .with_milliseconds(678);
    let actual = parse_time("012345.678").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_days_with_colon() {
    let expected = Time::default()
        .with_days(1)
        .with_hours(2)
        .with_minutes(34)
        .with_seconds(56)
        .with_milliseconds(789);
    let actual = parse_time("1 02:34:56.789").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_parses_days_short() {
    let expected = Time::default()
        .with_days(1)
        .with_hours(2)
        .with_minutes(34)
        .with_seconds(56)
        .with_milliseconds(789);
    let actual = parse_time("01 023456.789").unwrap();
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn it_doesnt_parse_invalid_input() {
    parse_time("nope").unwrap();
}

// TODO

#[test]
#[should_panic]
#[ignore]
fn it_doesnt_parse_invalid_days() {
    parse_time("lmao 0000.000").unwrap();
}

#[test]
#[should_panic]
#[ignore]
fn it_doesnt_parse_invalid_milliseconds() {
    parse_time("00 0000.lmao").unwrap();
}
