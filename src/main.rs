use std::env;
use std::fs;
use std::time::{Duration, Instant};

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You need to provide a day");
    }

    let day = args[1].clone().trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };

    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("{:02}", day_num));
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    let to_run = days::get_day(day_num);
    run_day("Part 1", to_run.0, input.clone());
    run_day("Part 2", to_run.1, input.clone());
}

fn run_day(name: &str, part_fn: days::DayFn, input: String) {
    println!("Running {}", name);
    let part1_start = Instant::now();
    if part_fn != days::noop {
        part_fn(input.clone());
    } else {
        println!("NOOP");
    }
    let part1_dur = part1_start.elapsed();
    println!("Took {}", fmt_dur(part1_dur));
}

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}