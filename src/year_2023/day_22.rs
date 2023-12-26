struct Point {
    x: u16,
    y: u16,
    z: u16,
}

struct Brick {
    start: Point,
    end: Point,
}

fn parse(input: &[u8]) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::with_capacity(7);
    let mut nums = [0u16; 6];
    let mut i = 0;
    for c in input {
        match c {
            b',' | b'~' => {
                i += 1;
            }
            b'\n' => {
                bricks.push(Brick {
                    start: Point {
                        x: nums[0],
                        y: nums[1],
                        z: nums[2],
                    },
                    end: Point {
                        x: nums[3],
                        y: nums[4],
                        z: nums[5],
                    },
                });
                nums = [0; 6];
                i = 0;
            }
            _ => nums[i] = nums[i] * 10 + (c - b'0') as u16,
        }
    }
    bricks
}

fn overlap(a: &Brick, b: &Brick) -> bool {
    !(a.start.x > b.end.x || a.end.x < b.start.x || a.start.y > b.end.y || a.end.y < b.start.y)
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    bricks.sort_unstable_by_key(|brick| brick.start.z);

    for i in 0..bricks.len() {
        let mut top = 0;
        for j in 0..i {
            if bricks[j].end.z > top
                && bricks[j].end.z < bricks[i].start.z
                && overlap(&bricks[i], &bricks[j])
            {
                top = bricks[j].end.z;
            }
        }
        let delta = bricks[i].start.z - top - 1;
        if delta > 0 {
            bricks[i].end.z -= delta;
            bricks[i].start.z -= delta;
        }
    }
}

fn disintegrate_brick(bricks: &mut Vec<Brick>) -> u16 {
    let mut supported: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];

    for (i, brick) in bricks.iter().enumerate() {
        for (j, other) in bricks.iter().enumerate() {
            if other.end.z == brick.start.z - 1 && overlap(brick, other) {
                supported[i].push(j);
            }
        }
    }

    let mut count = 0;
    for i in 0..supported.len() {
        if supported
            .iter()
            .all(|supports| supports.len() != 1 || supports[0] as usize != i)
        {
            count += 1;
        }
    }
    count
}

fn disintegrate_chain(bricks: &mut Vec<Brick>) -> usize {
    bricks.sort_unstable_by_key(|brick| brick.start.z);
    let mut supported_by: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];

    for (i, brick) in bricks.iter().enumerate() {
        for (j, other) in bricks.iter().enumerate() {
            if other.end.z + 1 == brick.start.z && overlap(brick, other) {
                supported_by[i].push(j);
            }
        }
    }

    let mut result = 0;
    for i in 0..bricks.len() {
        let mut deleted = vec![i];
        for j in i+1..bricks.len() {
            if !supported_by[j].is_empty() && supported_by[j].iter().all(|brick| deleted.contains(brick)) {
                deleted.push(j);
            }
        }
        result += deleted.len() - 1;
    }
    result
}

pub fn part1(input: &[u8]) -> String {
    let mut bricks = parse(input);
    settle_bricks(&mut bricks);
    disintegrate_brick(&mut bricks).to_string()
}

pub fn part2(input: &[u8]) -> String {
    let mut bricks = parse(input);
    settle_bricks(&mut bricks);
    disintegrate_chain(&mut bricks).to_string()
}
