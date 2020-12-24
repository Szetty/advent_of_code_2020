use std::collections::{HashMap, HashSet};

pub fn part1(inp: String) {
    println!("{}", parse_input_and_flip_initial_tiles(inp));
}

pub fn part2(inp: String) {
    println!("{}", parse_input_and_flip_tiles(inp));
}

type Tiles = Vec<Tile>;
type Tile = String;
type Coords = (i32, i32, i32);
const NEIGHBOUR_COORDS: [Coords; 6] = [
    (0, 1, -1),
    (1, 0, -1),
    (1, -1, 0),
    (0, -1, 1),
    (-1, 0, 1),
    (-1, 1, 0),
];

fn parse_input_and_flip_initial_tiles(inp: String) -> usize {
    flip_initial_tiles(parse_input(inp)).len()
}

fn parse_input_and_flip_tiles(inp: String) -> usize {
    flip_tiles(flip_initial_tiles(parse_input(inp))).len()
}

fn parse_input(inp: String) -> Tiles {
    inp.lines().map(&str::to_string).collect()
}

fn flip_initial_tiles(tiles: Tiles) -> HashSet<Coords> {
    let mut black_tiles: HashSet<Coords> = HashSet::new();
    for tile in tiles.into_iter() {
        let coords = navigate_tile(tile);
        if black_tiles.contains(&coords) {
            black_tiles.remove(&coords);
        } else {
            black_tiles.insert(coords);
        }
    }
    black_tiles
}

fn navigate_tile(tile: Tile) -> Coords {
    let mut current_coords = (0, 0, 0);
    let mut tile_iter = tile.chars();
    loop {
        let (x, y, z) = current_coords;
        match tile_iter.next() {
            Some('n') => match tile_iter.next() {
                Some('e') => current_coords = (x + 1, y, z - 1),
                Some('w') => current_coords = (x, y + 1, z - 1),
                _ => panic!("Unrecognized step"),
            },
            Some('s') => match tile_iter.next() {
                Some('e') => current_coords = (x, y - 1, z + 1),
                Some('w') => current_coords = (x - 1, y, z + 1),
                _ => panic!("Unrecognized step"),
            },
            Some('e') => current_coords = (x + 1, y - 1, z),
            Some('w') => current_coords = (x - 1, y + 1, z),
            None => return current_coords,
            _ => panic!("Unrecognized step"),
        }
    }
}

fn flip_tiles(mut black_tiles: HashSet<Coords>) -> HashSet<Coords> {
    for _ in 0..100 {
        black_tiles = do_flip_tiles(black_tiles);
    }
    black_tiles
}

fn do_flip_tiles(black_tiles: HashSet<Coords>) -> HashSet<Coords> {
    let mut white_tiles_with_black_neighbour_count: HashMap<Coords, usize> = HashMap::new();
    let mut new_black_tiles: HashSet<Coords> = HashSet::new();
    for black_tile in black_tiles.clone().iter() {
        let neighbours = compute_neighbours(*black_tile);
        let white_neighbours = neighbours
            .iter()
            .filter(|neighbour| !black_tiles.contains(neighbour))
            .cloned();
        let mut white_neighbours_count = 0;
        for white in white_neighbours {
            let current_black_count = *white_tiles_with_black_neighbour_count
                .get(&white)
                .unwrap_or(&0);
            white_tiles_with_black_neighbour_count.insert(white, current_black_count + 1);
            white_neighbours_count += 1;
        }
        let black_neighbours_count = neighbours.len() - white_neighbours_count;
        if black_neighbours_count == 1 || black_neighbours_count == 2 {
            new_black_tiles.insert(*black_tile);
        }
    }
    for (white_tile, _) in white_tiles_with_black_neighbour_count
        .into_iter()
        .filter(|(_, count)| *count == 2)
    {
        new_black_tiles.insert(white_tile);
    }
    new_black_tiles
}

fn compute_neighbours((x, y, z): Coords) -> HashSet<Coords> {
    let mut neighbours = HashSet::new();
    for (dx, dy, dz) in &NEIGHBOUR_COORDS {
        neighbours.insert((x + dx, y + dy, z + dz));
    }
    neighbours
}

#[test]
fn test_parse_input_and_flip_initial_tiles() {
    assert_eq!(
        10,
        parse_input_and_flip_initial_tiles(TEST_INPUT.to_string())
    );
}

#[test]
fn test_parse_input_and_flip_tiles() {
    assert_eq!(2208, parse_input_and_flip_tiles(TEST_INPUT.to_string()));
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;
