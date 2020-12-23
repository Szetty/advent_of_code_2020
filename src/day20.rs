use itertools::Itertools;
use na::{Column, DMatrix, DVector, Row, Transpose};
use regex::Regex;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};

pub fn part1(inp: String) {
    let value = parse_tiles_and_compute_border_product_of_matching(inp);
    assert_eq!(15405893262491, value);
    println!("{}", value);
}

pub fn part2(inp: String) {
    let image = parse_tiles_and_reconstruct_image(inp);
    println!("{}", search_sea_monsters_and_compute_water_roughness(image));
}

const VERSIONS_COUNT: usize = 8;
type Tiles = Vec<Tile>;
#[derive(Clone)]
struct Tile {
    id: TileID,
    size: usize,
    versions: TileVersions,
}
type TileID = i64;
type TileVersions = [TileVersion; VERSIONS_COUNT];
#[derive(Clone, Debug)]
struct TileVersion {
    grid: Grid,
}
impl TileVersion {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        Self {
            grid: Grid::new(grid),
        }
    }
    fn transpose(self) -> Self {
        Self {
            grid: self.grid.transpose(),
        }
    }
    fn flip_vertical(self) -> Self {
        Self {
            grid: self.grid.flip_vertical(),
        }
    }
    fn flip_horizontal(self) -> Self {
        Self {
            grid: self.grid.flip_horizontal(),
        }
    }
    fn north(self) -> TileBorder {
        self.grid.first_row()
    }
    fn south(self) -> TileBorder {
        self.grid.last_row()
    }
    fn west(self) -> TileBorder {
        self.grid.first_column()
    }
    fn east(self) -> TileBorder {
        self.grid.last_column()
    }
}
type TileBorder = DVector<u8>;
type MatchingType = (TileID, usize);
#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, Debug)]
struct Matching {
    m: HashMap<MatchingType, Vec<(MatchingType, Direction)>>,
}
impl Matching {
    fn new() -> Matching {
        Self { m: HashMap::new() }
    }
    fn add_match(&mut self, key: MatchingType, value: (MatchingType, Direction)) {
        let matches = self.m.get_mut(&key);
        match matches {
            Some(values) => values.push(value),
            None => {
                self.m.insert(key, vec![value]);
            }
        }
    }
    fn matches(self, key: MatchingType) -> Option<Vec<(MatchingType, Direction)>> {
        self.m.get(&key).cloned()
    }
}
impl std::fmt::Display for Matching {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut match_map: HashMap<TileID, Vec<(usize, Vec<(MatchingType, Direction)>)>> =
            HashMap::new();
        for ((tile_id, idx), value) in self.m.iter() {
            match match_map.get_mut(tile_id) {
                Some(vec) => vec.push((*idx, value.clone())),
                None => {
                    match_map.insert(*tile_id, vec![(*idx, value.clone())]);
                }
            }
        }
        let mut res = writeln!(f, "Matching:");
        for (tile_id, values) in match_map.iter() {
            res = res.and(writeln!(f, "  Tile {}:", tile_id));
            for (idx, value) in values.iter() {
                res = res.and(writeln!(f, "    {}: {:?}", idx, value));
            }
        }
        res.and(writeln!(f, ""))
    }
}
#[derive(Clone, Debug)]
struct SeaMonster {
    columns: usize,
    rows: usize,
    values: Vec<(usize, usize)>,
}
#[derive(Clone, Debug)]
struct Grid {
    grid: DMatrix<u8>,
}
impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        Self {
            grid: DMatrix::from_fn(grid.len(), grid[0].len(), |i, j| grid[i][j]),
        }
    }
    fn new_from_matrix(matrix: DMatrix<u8>) -> Self {
        Self {
            grid: matrix.clone(),
        }
    }
    fn transpose(self) -> Self {
        Self {
            grid: self.grid.transpose(),
        }
    }
    fn flip_vertical(self) -> Self {
        let mut new_grid = self.grid.clone();
        for j in 0..(new_grid.ncols() / 2) {
            let inversej = new_grid.ncols() - 1 - j;
            let colj = new_grid.column(j);
            let inversecolj = new_grid.column(inversej);
            new_grid.set_column(j, inversecolj);
            new_grid.set_column(inversej, colj);
        }
        Self { grid: new_grid }
    }
    fn flip_horizontal(self) -> Self {
        let mut new_grid = self.grid.clone();
        for i in 0..(new_grid.nrows() / 2) {
            let inversei = new_grid.nrows() - 1 - i;
            let coli = new_grid.row(i);
            let inversecoli = new_grid.row(inversei);
            new_grid.set_row(i, inversecoli);
            new_grid.set_row(inversei, coli);
        }
        Self { grid: new_grid }
    }
    fn first_row(self) -> TileBorder {
        self.grid.row(0)
    }
    fn last_row(self) -> TileBorder {
        self.grid.row(self.grid.nrows() - 1)
    }
    fn first_column(self) -> TileBorder {
        self.grid.column(0)
    }
    fn last_column(self) -> TileBorder {
        self.grid.column(self.grid.ncols() - 1)
    }
    fn count_non_zero(self) -> usize {
        self.grid
            .into_vector()
            .into_iter()
            .filter(|v| *v == 1u8)
            .count()
    }
}

fn parse_tiles_and_compute_border_product_of_matching(inp: String) -> i64 {
    let tiles = parse_tiles(inp);
    let matching = compute_matching(tiles.clone());
    return tiles
        .iter()
        .filter_map(|tile| corner_tile(tile, matching.clone()))
        .map(|(tile_id, _)| tile_id)
        .product::<i64>();
}

fn parse_tiles_and_reconstruct_image(inp: String) -> Grid {
    let tiles = parse_tiles(inp);
    let matching = compute_matching(tiles.clone());
    reconstruct_image(tiles, matching)
}

fn parse_tiles(inp: String) -> Tiles {
    let tile_id_regex: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    inp.split("\n\n")
        .map(|tile_str| {
            let mut tile_lines = tile_str.lines();
            let tile_id_line = tile_lines.next().unwrap();
            let tile_id = tile_id_regex
                .captures(tile_id_line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<TileID>()
                .unwrap();
            let tile_map: Vec<Vec<u8>> = tile_lines
                .map(|tile_line| {
                    tile_line
                        .chars()
                        .map(|c| match c {
                            '#' => 1,
                            '.' => 0,
                            _ => panic!("Unknown tile grid point"),
                        })
                        .collect()
                })
                .collect();
            Tile {
                id: tile_id,
                size: tile_map.len(),
                versions: build_tile_versions(tile_map),
            }
        })
        .collect()
}

fn build_tile_versions(grid: Vec<Vec<u8>>) -> TileVersions {
    let current_version = TileVersion::new(grid);
    let transposed = current_version.clone().transpose();
    [
        current_version.clone(),
        transposed.clone(),
        current_version.clone().flip_horizontal(),
        current_version.clone().flip_vertical(),
        current_version.clone().flip_horizontal().flip_vertical(),
        transposed.clone().flip_horizontal(),
        transposed.clone().flip_vertical(),
        transposed.clone().flip_horizontal().flip_vertical(),
    ]
}

fn compute_matching(tiles: Tiles) -> Matching {
    let mut matching = Matching::new();
    for i in 0..(tiles.len() - 1) {
        let tile1 = &tiles[i];
        for vi in 0..VERSIONS_COUNT {
            let tile_version1 = &tile1.versions[vi];
            for j in (i + 1)..tiles.len() {
                let tile2 = &tiles[j];
                for vj in 0..VERSIONS_COUNT {
                    let tile_version2 = &tile2.versions[vj];
                    let result =
                        match matching_direction(tile_version1.clone(), tile_version2.clone()) {
                            Some(Direction::North) => Some((Direction::North, Direction::South)),
                            Some(Direction::South) => Some((Direction::South, Direction::North)),
                            Some(Direction::West) => Some((Direction::West, Direction::East)),
                            Some(Direction::East) => Some((Direction::East, Direction::West)),
                            None => None,
                        };
                    match result {
                        Some((direction1, direction2)) => {
                            matching.add_match((tile1.id, vi), ((tile2.id, vj), direction1));
                            matching.add_match((tile2.id, vj), ((tile1.id, vi), direction2));
                        }
                        None => {}
                    }
                }
            }
        }
    }
    matching
}

fn matching_direction(tile_version1: TileVersion, tile_version2: TileVersion) -> Option<Direction> {
    if tile_version1.clone().north() == tile_version2.clone().south() {
        return Some(Direction::North);
    }
    if tile_version1.clone().south() == tile_version2.clone().north() {
        return Some(Direction::South);
    }
    if tile_version1.clone().west() == tile_version2.clone().east() {
        return Some(Direction::West);
    }
    if tile_version1.clone().east() == tile_version2.clone().west() {
        return Some(Direction::East);
    }
    None
}

fn corner_tile(tile: &Tile, matching: Matching) -> Option<(TileID, usize)> {
    let tile_id = tile.id;
    let mut is_corner_tile = None;
    for (i, _) in tile.versions.iter().enumerate() {
        let key = (tile_id.clone(), i);
        let matches = matching
            .clone()
            .matches(key)
            .map(|matches| matches.iter().map(|(tile_id, _)| tile_id).unique().count())
            .unwrap_or(0);
        if matches > 2 {
            return None;
        }
        if matches == 2 && is_corner_tile == None {
            is_corner_tile = Some((tile_id, i))
        }
    }
    return is_corner_tile;
}

fn reconstruct_image(tiles: Tiles, matching: Matching) -> Grid {
    let tiles_by_id = tiles
        .iter()
        .cloned()
        .map(|tile| (tile.id, tile))
        .collect::<HashMap<TileID, Tile>>();
    let image_size = (tiles.len() as f64).sqrt() as usize;
    let last_index = image_size - 1;
    let mut tile_grid: DMatrix<(TileID, usize)> =
        DMatrix::from_element(image_size, image_size, (0, 0));
    let mut unused_tiles: HashSet<TileID> = tiles.iter().map(|tile| tile.id).collect();
    let top_left_tiles = top_left_tiles(tiles.clone(), matching.clone());
    let mut current_tile = top_left_tiles.into_iter().next().unwrap();
    let mut next_index = (0, 0);
    let mut delta_index: i8 = 1;
    loop {
        // println!("{:?}", tile_grid);
        // println!("{:?}", next_index);
        tile_grid[next_index] = current_tile;
        unused_tiles.remove(&current_tile.0);
        if unused_tiles.len() == 0 {
            break;
        }
        let current_index = next_index.clone();
        let current_delta_index = delta_index.clone();
        match (next_index, delta_index) {
            ((i, j), 1) if i == last_index && j == last_index => {
                next_index = (image_size, image_size)
            }
            ((i, 0), -1) if i == last_index => next_index = (image_size, image_size),
            ((i, 0), -1) => {
                next_index = (i + 1, 0);
                delta_index = 1
            }
            ((i, j), 1) if j == last_index => {
                next_index = (i + 1, j);
                delta_index = -1
            }
            ((i, j), dj) => next_index = (i, (j as i8 + dj) as usize),
        };
        let matches = matching.clone().matches(current_tile).unwrap();
        let i = (*matches).iter().filter(|((tile_id, _), dir)| {
            let expected_direction = match (current_index.clone(), current_delta_index.clone()) {
                ((_, 0), 1) => Direction::East,
                ((_, j), -1) if j == last_index => Direction::West,
                ((_, j), _) if j == 0 || j == last_index => Direction::South,
                (_, 1) => Direction::East,
                (_, -1) => Direction::West,
                _ => panic!("Invalid next_index, delta_index state"),
            };
            *dir == expected_direction && unused_tiles.contains(tile_id)
        });
        current_tile = i.clone().next().unwrap().0;
    }
    build_image_from_tiles(tiles_by_id, tile_grid)
}

fn top_left_tiles(tiles: Tiles, matching: Matching) -> Vec<(TileID, usize)> {
    let mut corner_tiles = vec![];
    for tile in tiles.iter() {
        let mut temp_corner_tiles = vec![];
        let mut has_other_tile = false;
        for (i, _) in tile.versions.iter().enumerate() {
            let key = (tile.id.clone(), i);
            let matches = matching
                .clone()
                .matches(key)
                .filter(|matches| {
                    matches.iter().any(|(_, dir)| *dir == Direction::East)
                        && matches.iter().any(|(_, dir)| *dir == Direction::South)
                })
                .map(|matches| matches.iter().map(|(tile_id, _)| tile_id).unique().count())
                .unwrap_or(0);
            if matches > 2 {
                has_other_tile = true;
                break;
            }
            if matches == 2 {
                temp_corner_tiles.push((tile.id.clone(), i.clone()));
            }
        }
        if !has_other_tile {
            corner_tiles.append(&mut temp_corner_tiles.clone());
        }
    }
    corner_tiles
}

fn build_image_from_tiles(
    tiles_by_id: HashMap<TileID, Tile>,
    tile_grid: DMatrix<(TileID, usize)>,
) -> Grid {
    let tiles_size = tiles_by_id.values().next().unwrap().size;
    let mut image: DMatrix<u8> = DMatrix::new_zeros(
        tile_grid.nrows() * (tiles_size - 2),
        tile_grid.nrows() * (tiles_size - 2),
    );
    let mut image_i = 0;
    let mut image_j = 0;
    for i in 0..(tile_grid.nrows() * tiles_size) {
        let tile_i = i / tiles_size;
        let grid_i = i % tiles_size;
        if grid_i == 0 || grid_i == tiles_size - 1 {
            continue;
        }
        for j in 0..(tile_grid.nrows() * tiles_size) {
            let tile_j = j / tiles_size;
            let grid_j = j % tiles_size;
            if grid_j == 0 || grid_j == tiles_size - 1 {
                continue;
            }
            let (tile_id, idx) = tile_grid[(tile_i, tile_j)];
            let tile = tiles_by_id.get(&tile_id).unwrap();
            image[(image_i, image_j)] = tile.versions[idx].grid.grid[(grid_i, grid_j)];
            image_j += 1;
        }
        image_i += 1;
        image_j = 0;
    }
    Grid::new_from_matrix(image)
}

#[allow(dead_code)]
fn image_to_string(grid: Grid) -> String {
    let mut result = "".to_string();
    for i in 0..grid.grid.nrows() {
        let mut line = "".to_string();
        for j in 0..grid.grid.ncols() {
            line += &(if grid.grid[(i, j)] == 1 { "#" } else { "." }).to_string();
        }
        result += &(line + "\n");
    }
    result
}

fn search_sea_monsters_and_compute_water_roughness(image: Grid) -> usize {
    let sea_monster = parse_sea_monster();
    let image_version_fns: [fn(Grid) -> Grid; 8] = [
        |image| image,
        |image| image.transpose(),
        |image| image.flip_horizontal(),
        |image| image.flip_vertical(),
        |image| image.flip_horizontal().flip_vertical(),
        |image| image.transpose().flip_horizontal(),
        |image| image.transpose().flip_vertical(),
        |image| image.transpose().flip_horizontal().flip_vertical(),
    ];
    let rows = image.grid.nrows() - sea_monster.clone().rows;
    let columns = image.grid.ncols() - sea_monster.clone().columns;
    for image_version_fn in &image_version_fns {
        let image_version = image_version_fn(image.clone());
        let mut sea_monsters_found: usize = 0;
        for i in 0..rows {
            for j in 0..columns {
                if matches_sea_monster(image_version.clone(), (i, j), sea_monster.clone()) {
                    sea_monsters_found += 1;
                }
            }
        }
        if sea_monsters_found > 0 {
            return image.count_non_zero() - sea_monsters_found * sea_monster.values.len();
        }
    }
    0
}

fn parse_sea_monster() -> SeaMonster {
    let sea_monster_lines = SEA_MONSTER.lines().collect::<Vec<&str>>();
    let values = sea_monster_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i, j), c))
                .filter(|(_, c)| *c == '#')
                .map(|(idx, _)| idx)
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();
    SeaMonster {
        columns: sea_monster_lines[0].len(),
        rows: sea_monster_lines.len(),
        values: values,
    }
}

fn matches_sea_monster(image: Grid, (i, j): (usize, usize), sea_monster: SeaMonster) -> bool {
    for (di, dj) in sea_monster.values.iter() {
        if image.grid[(i + di, j + dj)] != 1 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_parse_tiles_and_compute_border_product_of_matching() {
    assert_eq!(
        20899048083289,
        parse_tiles_and_compute_border_product_of_matching(TEST_INPUT.to_string())
    )
}

#[test]
fn test_parse_tiles_and_reconstruct_image_and_compute_water_roughness() {
    let image = parse_tiles_and_reconstruct_image(TEST_INPUT.to_string());
    assert_eq!(TEST_IMAGE, image_to_string(image.clone()));
    assert_eq!(273, search_sea_monsters_and_compute_water_roughness(image));
}

const SEA_MONSTER: &str = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

#[allow(dead_code)]
const TEST_IMAGE: &str = r#".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###
"#;
