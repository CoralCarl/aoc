use std::collections::{HashMap, HashSet};

type Point = (i16, i16);

#[derive(Default, Debug)]
struct Map {
    tiles: HashMap<Point, u8>,
    start: Point,
    end: Point,
}

fn parse(input: &[u8]) -> Map {
    let mut map = Map::default();
    let mut row = 0;
    let mut col = 0;

    for c in input {
        match c {
            b'#' => {}
            b'\n' => {
                col = -1;
                row += 1;
            }
            _ => {
                map.tiles.insert((col, row), *c);
            }
        }
        col += 1;
    }

    map.start = (1, 0);
    map.end = *map.tiles.keys().max_by_key(|&(_, y)| y).unwrap();

    map
}

type Branches = HashMap<Point, HashMap<Point, u32>>;

fn generate_paths(map: &Map) -> Branches {
    let mut branches: Branches = HashMap::new();
    let mut pos = map.start;
    let mut d = (0, 1);
    let mut l = 1; // maybe 0

    while map.tiles[&pos] == b'.' {
        for dn in [d, (d.1, -d.0), (-d.1, d.0)] {
            let next = (pos.0 + dn.0, pos.1 + dn.1);
            if map.tiles.contains_key(&next) {
                pos.0 += dn.0;
                pos.1 += dn.1;
                d = dn;
                break;
            }
        }
        l += 1;
    }
    pos.0 += d.0;
    pos.1 += d.1;

    branches.entry(map.start).or_default().insert(pos, l);
    branch(&map, &mut branches, pos);

    branches
}

fn branch(map: &Map, branches: &mut Branches, start: (i16, i16)) {
    for (mut d, valid) in [
        ((1, 0), b'>'),
        ((0, 1), b'v'),
        ((-1, 0), b'<'),
        ((0, -1), b'^'),
    ] {
        let mut pos = (start.0 + d.0, start.1 + d.1);
        if !map.tiles.contains_key(&pos) || map.tiles[&pos] != valid {
            continue;
        }
        pos.0 += d.0;
        pos.1 += d.1;
        let mut l = 3;

        'walk: while map.tiles[&pos] == b'.' {
            for dn in [d, (d.1, -d.0), (-d.1, d.0)] {
                let next = (pos.0 + dn.0, pos.1 + dn.1);
                if next == map.end {
                    d = dn;
                    break 'walk;
                }
                if map.tiles.contains_key(&next) {
                    pos.0 += dn.0;
                    pos.1 += dn.1;
                    d = dn;
                    break;
                }
            }
            l += 1;
        }
        pos.0 += d.0;
        pos.1 += d.1;

        branches.entry(start).or_default().insert(pos, l);

        if !branches.contains_key(&pos) {
            branch(map, branches, pos);
        }
    }
}

fn longest_hike(
    start: &Point,
    end: &Point,
    branches: &Branches,
    visited: HashSet<Point>,
    length: u32,
) -> u32 {
    if start == end {
        return length;
    }
    let mut best = 0;
    for (next, len) in &branches[start] {
        if !visited.contains(next) {
            let mut visited = visited.clone();
            visited.insert(*next);
            let hike = longest_hike(next, end, branches, visited, length + len);
            if hike > best {
                best = hike;
            }
        }
    }
    best
}

pub fn part1(input: &[u8]) -> String {
    let map = parse(input);
    let branches = generate_paths(&map);
    let best = longest_hike(&map.start, &map.end, &branches, HashSet::new(), 0);
    best.to_string()
}

pub fn part2(input: &[u8]) -> String {
    let map = parse(input);
    let branches = generate_paths(&map);
    let mut new_branches = branches.clone();
    for (start, ends) in &branches {
        for (end, length) in ends {
            new_branches.entry(*end).or_default().insert(*start, *length);
        }
    }
    let best = longest_hike(&map.start, &map.end, &new_branches, HashSet::new(), 0);
    best.to_string()
}
