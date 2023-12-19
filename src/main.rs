use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

mod year_2023;

type Solution = (Box<dyn Fn(&[u8]) -> String>, Box<dyn Fn(&[u8]) -> String>);

fn main() {
    let mut args: Vec<String> = Vec::new();
    let mut iargs = std::env::args().skip(1);
    let mut rounds = 1;
    let mut input = String::new();

    while let Some(arg) = iargs.next() {
        match arg.as_str() {
            "-r" => {
                rounds = iargs
                    .next()
                    .expect("format: -r rounds")
                    .parse::<usize>()
                    .unwrap();
            }
            "-d" => {
                input = iargs.next().expect("format: -d 'inputdata'");
            }
            "-t" => {
                input =
                    String::from_utf8(std::fs::read("test.txt").expect("unknown file"))
                        .unwrap();
            }
            _ => args.push(arg),
        }
    }

    let mut years: HashMap<usize, HashMap<usize, Solution>> = HashMap::new();

    years.insert(2023, year_2023::days());

    let year: usize;
    let mut days: Vec<usize> = Vec::new();

    match args.len() {
        0 => {
            year = *years.keys().max().unwrap();
            days.push(*years.get(&year).unwrap().keys().max().unwrap());
        }
        1 => {
            year = args[0].parse::<usize>().expect("invalid year format");
            for &day in years.get(&year).unwrap().keys() {
                days.push(day);
            }
            days.sort();
        }
        2 => {
            year = args[0].parse::<usize>().expect("invalid year format");
            days.push(args[1].parse::<usize>().expect("invalid day format"));
        }
        _ => panic!("expected arguments: [yyyy [dd]]"),
    }

    println!("========  {year}  ========");
    for day in &days {
        let solution = years.get_mut(&year).unwrap().remove(day).unwrap();
        let input_p: String;
        if input.is_empty() {
            input_p = utils::input::read_input(year, *day);
        } else {
            input_p = input.clone();
        }
        run_solution(*day, solution, input_p.as_bytes(), rounds);
    }
}

fn format_time(time: &Duration) -> String {
    let s;
    if time.as_secs() >= 60 {
        let mins: u64 = time.as_secs() / 60;
        let secs: u64 = time.as_secs() - mins * 60;
        s = format!("{:2}m{:02}s", mins, secs);
    } else if time.as_secs() > 0 {
        s = format!("{:2}.{:02}s", time.as_secs(), time.subsec_millis() / 10);
    } else if time.as_millis() > 0 {
        s = format!("{:4}ms", time.as_millis());
    } else if time.as_micros() > 0 {
        s = format!("{:4}μs", time.as_micros());
    } else {
        s = format!("{:4}ns", time.as_nanos());
    }
    s
}

fn print_result(part: usize, result: &str, time: Duration) {
    let mut result = result.lines();

    println!(
        "{} - Part {}: {}",
        format_time(&time),
        part,
        result.next().unwrap()
    );

    while let Some(line) = result.next() {
        println!("                 {}", line);
    }
}

fn run_solution(day: usize, solution: Solution, input: &[u8], rounds: usize) {
    println!("-------- Day {:02} --------", day);
    let instant = Instant::now();
    let part1 = solution.0(input);
    for _ in 0..rounds - 1 {
        solution.0(input);
    }
    let time = instant.elapsed() / rounds as u32;
    print_result(1, &part1, time);

    let instant = Instant::now();
    let part2 = solution.1(input);
    for _ in 0..rounds - 1 {
        solution.1(input);
    }
    let time = instant.elapsed() / rounds as u32;
    print_result(2, &part2, time);

    println!();
}
