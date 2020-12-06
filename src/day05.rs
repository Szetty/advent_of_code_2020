use std::str::Chars;

pub fn part1(inp: String) {
    let max_seat_id = inp
        .lines()
        .map(|l| {
            let boarding_pass = decode_boarding_pass(l);
            boarding_pass.seat_id
        })
        .max()
        .unwrap();
    println!("{}", max_seat_id);
}

pub fn part2(inp: String) {
    let mut seat_ids: Vec<i32> = inp
        .lines()
        .map(|l| {
            let boarding_pass = decode_boarding_pass(l);
            boarding_pass.seat_id
        })
        .collect::<Vec<i32>>();
    seat_ids.sort();
    for i in 1..seat_ids.len() {
        if seat_ids[i] != seat_ids[i - 1] + 1 {
            println!("{}", seat_ids[i - 1] + 1);
            return;
        }
    }
}

#[derive(std::cmp::PartialEq, Debug)]
struct BoardingPass {
    row: i32,
    column: i32,
    seat_id: i32,
}

fn decode_boarding_pass(encoded_boarding_pass: &str) -> BoardingPass {
    let (encoded_row, encoded_column) = encoded_boarding_pass.split_at(7);
    let row = decode_chars((0, 127), encoded_row.chars());
    let column = decode_chars((0, 7), encoded_column.chars());
    BoardingPass {
        row: row,
        column: column,
        seat_id: row * 8 + column,
    }
}

fn decode_chars(mut range: (i32, i32), encoded_chars: Chars<'_>) -> i32 {
    let initial_range = range;
    for c in encoded_chars {
        let mid = ((range.0 + range.1) as f32) / 2.0;
        match c {
            'B' | 'R' => range = (mid.ceil() as i32, range.1),
            'F' | 'L' => range = (range.0, mid.floor() as i32),
            _ => panic!("[decode_chars] Unknown char {} {:?}", c, initial_range),
        }
    }
    if range.0 != range.1 {
        panic!("[decode_chars] Not equal: {:?} {:?}", range, initial_range);
    }
    return range.0;
}

#[test]
fn test_decode_boarding_pass() {
    assert_eq!(
        decode_boarding_pass("FBFBBFFRLR"),
        BoardingPass {
            row: 44,
            column: 5,
            seat_id: 357
        }
    );
    assert_eq!(
        decode_boarding_pass("BFFFBBFRRR"),
        BoardingPass {
            row: 70,
            column: 7,
            seat_id: 567
        }
    );
    assert_eq!(
        decode_boarding_pass("FFFBBBFRRR"),
        BoardingPass {
            row: 14,
            column: 7,
            seat_id: 119
        }
    );
    assert_eq!(
        decode_boarding_pass("BBFFBBFRLL"),
        BoardingPass {
            row: 102,
            column: 4,
            seat_id: 820
        }
    );
}
