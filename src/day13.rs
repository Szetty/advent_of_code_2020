use modinverse::egcd;
use num::integer::lcm;
use std::ops::Add;

pub fn part1(inp: String) {
    let (earliest_time, buses) = parse_input(inp);
    println!("{}", find_bus(earliest_time, buses));
}

pub fn part2(inp: String) {
    println!("{}", parse_input_and_find_time(inp));
}

fn parse_input_and_find_time(inp: String) -> i64 {
    let (_, buses) = parse_input(inp);
    find_time(buses)
}

fn parse_input(inp: String) -> (i64, Vec<Option<i64>>) {
    match inp.lines().collect::<Vec<&str>>()[..] {
        [earliest_time_str, buses_str] => {
            let earliest_time = earliest_time_str.parse().unwrap();
            let buses = buses_str
                .split(",")
                .map(|bus_str| match bus_str.parse() {
                    Ok(bus) => Some(bus),
                    _ => None,
                })
                .collect();
            (earliest_time, buses)
        }
        _ => panic!("Input in wrong format"),
    }
}

fn find_bus(earliest_time: i64, buses: Vec<Option<i64>>) -> i64 {
    let mut current_time = earliest_time;
    loop {
        let bus_found = buses.iter().find(|&bus_option| match bus_option {
            Some(bus) => current_time % bus == 0,
            None => false,
        });
        match bus_found {
            Some(bus) => return bus.unwrap() * (current_time - earliest_time),
            None => current_time = current_time + 1,
        }
    }
}

struct PeriodicSystem {
    phase: i64,
    period: i64,
}

impl Add<PeriodicSystem> for PeriodicSystem {
    type Output = PeriodicSystem;
    fn add(self, other: PeriodicSystem) -> PeriodicSystem {
        let (g, u, _) = egcd(self.period, other.period);
        assert_eq!(0, (self.phase - other.phase).abs() % g);
        let period = lcm(self.period, other.period);
        let mut phase: i128 = self.period as i128 * u as i128 * (other.phase - self.phase) as i128
            + self.phase as i128;
        while phase < 0 {
            phase = phase + period as i128
        }
        phase = phase % period as i128;
        PeriodicSystem {
            phase: phase as i64,
            period: period,
        }
    }
}

fn find_time(buses: Vec<Option<i64>>) -> i64 {
    let a: PeriodicSystem = buses
        .iter()
        .enumerate()
        .filter_map(|(i, &bus_option)| {
            bus_option.map(|bus| PeriodicSystem {
                period: bus,
                phase: (buses.len() - 1 - i) as i64,
            })
        })
        .fold(None, |acc, periodic_system| match acc {
            Some(acc_periodic_system) => Some(acc_periodic_system + periodic_system),
            None => Some(periodic_system),
        })
        .unwrap();
    return a.phase + 1 - buses.len() as i64;
}

#[test]
fn test_parse_input_and_find_time() {
    assert_eq!(
        191659,
        parse_input_and_find_time(
            "0\n641,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,661".to_string()
        )
    );
    assert_eq!(3417, parse_input_and_find_time("0\n17,x,13,19".to_string()));
    assert_eq!(
        754018,
        parse_input_and_find_time("0\n67,7,59,61".to_string())
    );
    assert_eq!(
        779210,
        parse_input_and_find_time("0\n67,x,7,59,61".to_string())
    );
    assert_eq!(
        1261476,
        parse_input_and_find_time("0\n67,7,x,59,61".to_string())
    );
    assert_eq!(
        1202161486,
        parse_input_and_find_time("0\n1789,37,47,1889".to_string())
    );
}
