extern crate timecalc;

use std::env;
use timecalc::Time;

fn main() {
    match run() {
        Ok(res) => {
            println!("{}", res);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> timecalc::TimeResult<String> {
    Ok(match parse_args()? {
        Action::Help => "USAGE".to_string(),
        Action::Parse(time) => format!("{}", time),
        Action::Millis(time) => format!("{}", time.as_milliseconds()),
    })
}

enum Action {
    Help,
    Parse(Time),
    Millis(Time),
}

fn parse_args() -> timecalc::TimeResult<Action> {
    let mkerr =
        |s: String| -> timecalc::TimeError { timecalc::TimeError::from(s) };

    let mut args = env::args().skip(1);

    match args.next().as_ref() {
        Some(s) if s.as_str() == "parse" => args
            .next()
            .ok_or(mkerr(
                "Action \"parse\" requires the time value as argument"
                    .to_string(),
            ))
            .and_then(|arg_time| {
                Time::new(arg_time).map(|time| Action::Parse(time))
            }),

        Some(s) if s.as_str() == "millis" => args
            .next()
            .ok_or(mkerr(
                "Action \"millis\" requires the time value as argument"
                    .to_string(),
            ))
            .and_then(|arg_time| {
                Time::new(arg_time).map(|time| Action::Millis(time))
            }),

        Some(s) => (mkerr(format!("Failed to parse action {}", s))).into(),

        None => Ok(Action::Help),
    }
}
