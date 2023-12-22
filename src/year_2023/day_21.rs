use std::collections::HashSet;

type GridSize = u8;
type Point = (GridSize, GridSize);

#[derive(Default, Debug)]
struct Map {
    start: Point,
    rocks: HashSet<Point>,
    width: GridSize,
    height: GridSize,
}

fn parse(input: &[u8]) -> Map {
    let mut map = Map::default();

    for c in input.iter().take(input.len() - 1) {
        match c {
            b'S' => {
                map.start = (map.width, map.height);
                map.width += 1;
            }
            b'#' => {
                map.rocks.insert((map.width, map.height));
                map.width += 1;
            }
            b'\n' => {
                map.width = 0;
                map.height += 1;
            }
            _ => {
                map.width += 1;
            }
        }
    }
    map.height += 1;
    map
}
pub fn part1(input: &[u8]) -> String {
    let map = parse(input);

    let mut even: HashSet<Point> = HashSet::new();
    let mut next: HashSet<Point> = HashSet::from([map.start]);

    for _ in 0..64 / 2 + 1 {
        let current = next;
        next = HashSet::new();
        for p in &current {
            if !even.contains(p) {
                even.insert(*p);
                for pd in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x = p.0.wrapping_add_signed(pd.0);
                    let y = p.1.wrapping_add_signed(pd.1);
                    if x < map.width && y < map.height && !map.rocks.contains(&(x, y)) {
                        next.insert((x, y));
                    }
                }
            }
        }
        let current = next;
        next = HashSet::new();
        for p in &current {
            for pd in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let x = p.0.wrapping_add_signed(pd.0);
                let y = p.1.wrapping_add_signed(pd.1);
                if x < map.width && y < map.height && !map.rocks.contains(&(x, y)) {
                    next.insert((x, y));
                }
            }
        }
    }

    // for y in 0..map.height {
    //     for x in 0..map.width {
    //         let p = (x, y);
    //         if even.contains(&p) {
    //             print!("O");
    //         } else if map.rocks.contains(&p) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    
    println!("{} {}", map.width, map.height);

    even.len().to_string()
}

pub fn part2(input: &[u8]) -> String {
    String::from("not implemented")
}
