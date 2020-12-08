#[path = "day01.rs"]
mod day01;
#[path = "day02.rs"]
mod day02;
#[path = "day03.rs"]
mod day03;
#[path = "day04.rs"]
mod day04;
#[path = "day05.rs"]
mod day05;
#[path = "day06.rs"]
mod day06;
#[path = "day07.rs"]
mod day07;
#[path = "day08.rs"]
mod day08;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        7 => (day07::part1, day07::part2),
        8 => (day08::part1, day08::part2),
        9 => (noop, noop),
        10 => (noop, noop),
        11 => (noop, noop),
        12 => (noop, noop),
        13 => (noop, noop),
        14 => (noop, noop),
        15 => (noop, noop),
        16 => (noop, noop),
        17 => (noop, noop),
        18 => (noop, noop),
        19 => (noop, noop),
        20 => (noop, noop),
        21 => (noop, noop),
        22 => (noop, noop),
        23 => (noop, noop),
        24 => (noop, noop),
        25 => (noop, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
