use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use utils::structure::{Solution, Year};

mod year_2015;
use year_2015::*;
mod year_2023;
use year_2023::*;

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

fn print_day(id: usize, day: &mut dyn Solution) {
    println!("-------- Day {:02} --------", id);

    let instant = Instant::now();
    day.parse();
    let time = instant.elapsed();
    println!("{} - Parsing", format_time(&time));

    let instant = Instant::now();
    let part1 = day.part1();
    let time = instant.elapsed();
    print_result(1, &part1, time);

    let instant = Instant::now();
    let part2 = day.part2();
    let time = instant.elapsed();
    print_result(2, &part2, time);

    println!();
}

fn print_year(id: usize, year: &mut dyn Year) {
    println!("========  {id}  ========");

    for i in 1..=25 {
        if let Some(day) = year.get_days().get_mut(&i) {
            print_day(i, day.as_mut());
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let years: [(usize, Box<dyn Year>); 2] = [
        (2015, Box::new(Year2015::new())),
        (2023, Box::new(Year2023::new())),
    ];
    let mut years = HashMap::from(years);

    match args.len() {
        1 => {
            let year_id = years.keys().max().unwrap().clone();
            let day_id = years
                .get_mut(&year_id)
                .unwrap()
                .get_days()
                .keys()
                .max()
                .unwrap()
                .clone();
            let day = years
                .get_mut(&year_id)
                .unwrap()
                .get_days()
                .get_mut(&day_id)
                .unwrap()
                .as_mut();
            print_day(day_id, day);
        }
        2 => {
            let year_id = args[1].parse::<usize>().expect("year parameter invalid");
            let year = years.get_mut(&year_id).expect("year not implemented").as_mut();
            print_year(year_id, year);
        }
        3 => {
            let year_id = args[1].parse::<usize>().expect("year parameter invalid");
            let day_id = args[2].parse::<usize>().expect("day parameter invalid");
            let year = years.get_mut(&year_id).expect("year not implemented");
            let day = year
                .get_days()
                .get_mut(&day_id)
                .expect("day not implemented")
                .as_mut();
            print_day(day_id, day);
        }
        _ => panic!("expected arguments: yyyy [dd]"),
    }
}
