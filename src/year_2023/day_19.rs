use std::collections::HashMap;

pub fn part1(input: &[u8]) -> String {
    let (wfs, ratings) = parse(input);
    let mut result = 0;
    for rating in ratings {
        let mut register = "in";

        while let Some(rules) = wfs.get(register) {
            for (idx, op, val, out) in rules {
                if *idx == 5 {
                    register = out;
                } else {
                    if *op == '>' {
                        if rating[*idx as usize] > *val {
                            register = out;
                            break;
                        }
                    } else {
                        if rating[*idx as usize] < *val {
                            register = out;
                            break;
                        }
                    }
                }
            }
        }
        if register == "A" {
            result += rating.iter().sum::<usize>();
        }
    }
    result.to_string()
}

pub fn part2(input: &[u8]) -> String {
    let (wfs, _) = parse(input);
    let bounds = bounds(&wfs, "in", [(1, 4000); 4]);
    let mut result = 0;

    for ranges in bounds {
        result += ranges.iter().map(|(a, b)| b - a + 1).product::<usize>();
    }

    result.to_string()
}

type Workflows = HashMap<String, Vec<(u8, char, usize, String)>>;

fn bounds(wfs: &Workflows, node: &str, range: [(usize, usize); 4]) -> Vec<[(usize, usize); 4]> {
    let mut accepted_ranges: Vec<[(usize, usize); 4]> = Vec::new();

    let mut thinned_range = range;
    for (attr, op, val, out) in wfs.get(node).unwrap() {
        if out == "R" {
            if *op == '>' {
                thinned_range[*attr as usize].1 = *val;
            } else if *op == '<' {
                thinned_range[*attr as usize].0 = *val;
            }

            continue;
        }
        let mut range = thinned_range;
        if *op == '>' {
            range[*attr as usize].0 = *val + 1;
            thinned_range[*attr as usize].1 = *val;
        } else if *op == '<' {
            range[*attr as usize].1 = *val - 1;
            thinned_range[*attr as usize].0 = *val;
        }
        if out == "A" {
            accepted_ranges.push(range);
        } else {
            accepted_ranges.extend(bounds(wfs, out, range));
        }
    }

    accepted_ranges
}

fn parse(input: &[u8]) -> (Workflows, Vec<[usize; 4]>) {
    let mut wfs: Workflows = HashMap::new();
    let mut lines = std::str::from_utf8(input).unwrap().lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (k, rules) = line.split_once('{').unwrap();
        let mut v: Vec<(u8, char, usize, String)> = Vec::new();
        let rules = rules[..rules.len() - 1].split(',');
        for rule in rules {
            if let Some((wf, out)) = rule.split_once(':') {
                let op;
                if let Some((r, n)) = if wf.contains('<') {
                    op = '<';
                    wf.split_once('<')
                } else {
                    op = '>';
                    wf.split_once('>')
                } {
                    v.push((
                        match r {
                            "x" => 0,
                            "m" => 1,
                            "a" => 2,
                            "s" => 3,
                            _ => panic!(),
                        },
                        op,
                        n.parse::<usize>().unwrap(),
                        out.to_string(),
                    ));
                }
            } else {
                v.push((5, ' ', 0, rule.to_string()));
            }
        }
        for i in (1..v.len()).rev() {
            if v[i].3 == v[i-1].3 && v[i].0 == 5 {
                v.swap_remove(i-1);
            } else {
                break
            }
        }
        wfs.insert(k.to_string(), v);
    }

    let mut ratings: Vec<[usize; 4]> = Vec::new();
    while let Some(line) = lines.next() {
        let mut attributes = [0; 4];

        for (i, a) in line[1..line.len() - 1].split(',').enumerate() {
            attributes[i] = a[2..].parse::<usize>().unwrap();
        }
        ratings.push(attributes);
    }
    (wfs, ratings)
}
