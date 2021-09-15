#![cfg(test)]

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
