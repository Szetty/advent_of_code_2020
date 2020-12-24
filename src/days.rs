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
#[path = "day09.rs"]
mod day09;
#[path = "day10.rs"]
mod day10;
#[path = "day11.rs"]
mod day11;
#[path = "day12.rs"]
mod day12;
#[path = "day13.rs"]
mod day13;
#[path = "day14.rs"]
mod day14;
#[path = "day15.rs"]
mod day15;
#[path = "day16.rs"]
mod day16;
#[path = "day17.rs"]
mod day17;
#[path = "day18.rs"]
mod day18;
#[path = "day19.rs"]
mod day19;
#[path = "day20.rs"]
mod day20;
#[path = "day21.rs"]
mod day21;
#[path = "day22.rs"]
mod day22;
#[path = "day23.rs"]
mod day23;
#[path = "day24.rs"]
mod day24;
#[path = "day25.rs"]
mod day25;

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
        9 => (day09::part1, day09::part2),
        10 => (day10::part1, day10::part2),
        11 => (day11::part1, day11::part2),
        12 => (day12::part1, day12::part2),
        13 => (day13::part1, day13::part2),
        14 => (day14::part1, day14::part2),
        15 => (day15::part1, day15::part2),
        16 => (day16::part1, day16::part2),
        17 => (day17::part1, day17::part2),
        18 => (day18::part1, day18::part2),
        19 => (day19::part1, day19::part2),
        20 => (day20::part1, day20::part2),
        21 => (day21::part1, day21::part2),
        22 => (day22::part1, day22::part2),
        23 => (day23::part1, day23::part2),
        24 => (day24::part1, day24::part2),
        25 => (day25::part1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
