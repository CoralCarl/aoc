pub fn part1(input: &[u8]) -> String {
    let mut plan: Vec<(char, usize)> = Vec::new();
    for line in std::str::from_utf8(input).unwrap().lines() {
        let mut spl = line.split_whitespace();
        plan.push((
            spl.next().unwrap().chars().next().unwrap(),
            spl.next().unwrap().parse::<usize>().unwrap(),
        ));
    }
    area(&plan).to_string()
}

pub fn part2(input: &[u8]) -> String {
    let mut plan: Vec<(char, usize)> = Vec::new();
    for line in std::str::from_utf8(input).unwrap().lines() {
        let (v, d) = line.split_once('#').unwrap().1.split_at(5);
        plan.push((
            ['R', 'D', 'L', 'U'][d.chars().next().unwrap().to_digit(10).unwrap() as usize],
            usize::from_str_radix(&v, 16).unwrap(),
        ));
    }
    area(&plan).to_string()
}

fn area(plan: &Vec<(char, usize)>) -> i64 {
    let mut vertices = Vec::from([(0, 0)]);

    let mut x = 0i64;
    let mut y = 0i64;

    for &(d, v) in plan {
        match d {
            'R' => x += v as i64,
            'D' => y += v as i64,
            'L' => x -= v as i64,
            'U' => y -= v as i64,
            _ => panic!(),
        };

        vertices.push((x, y));
    }

    vertices.push(vertices[0]);

    vertices
        .iter()
        .zip(vertices.iter().skip(1))
        .map(|(v1, v2)| v1.0 * v2.1 - v1.1 * v2.0 + (v2.0 - v1.0 + v2.1 - v1.1).abs())
        .sum::<i64>()
        / 2
        + 1
}
