use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use utils::Solution;

mod year_2015;
// mod year_2023;

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

fn print_day(day: usize, input: String, solution: &mut dyn Fn() -> Box<dyn Solution>) {
    println!("-------- Day {:02} --------", day);
    let mut solution = solution();

    let instant = Instant::now();
    solution.parse(input);
    let time = instant.elapsed();
    println!("{} - Parsing", format_time(&time));

    let instant = Instant::now();
    let part1 = solution.part1();
    let time = instant.elapsed();
    print_result(1, &part1, time);

    let instant = Instant::now();
    let part2 = solution.part2();
    let time = instant.elapsed();
    print_result(2, &part2, time);

    println!();
}

fn run_solution(year: usize, day: usize, solution: &mut dyn Fn() -> Box<dyn Solution>) {
    let input = utils::input::read_input(year, day);
    print_day(day, input, solution);
}

fn run_test(year: usize, day: usize, test: &str, solution: &mut dyn Fn() -> Box<dyn Solution>) {
    let input = utils::input::read_test_input(year, day, test);
    print_day(day, input, solution);
}

fn main() {
    let mut args: Vec<String> = Vec::new();
    let mut kwargs: HashMap<String, String> = HashMap::new();
    let mut iargs = std::env::args().skip(1);

    while let Some(arg) = iargs.next() {
        match arg.as_str() {
            "-t" => {
                kwargs.insert(arg, iargs.next().expect("argument -t requires index"));
            }
            _ => args.push(arg),
        }
    }

    let mut years: HashMap<usize, HashMap<usize, Box<dyn Fn() -> Box<dyn Solution>>>> =
        HashMap::new();

    years.insert(2015, year_2015::days());

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
    for &day in &days {
        let mut solution = years
            .get_mut(&year)
            .unwrap()
            .remove(&day)
            .unwrap();
        if let Some(test) = kwargs.get("-t") {
            run_test(year, day, test, &mut solution);
        } else {
            run_solution(year, day, &mut solution);
        }
    }
}
