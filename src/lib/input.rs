use std::fs::File;
use std::io::prelude::*;
const TOKEN: &str = "53616c7465645f5f572d38c54040f242a65b98b4bdbd09b842cb33c29bf7a4838095a854b3699265d56d731a716a2ed6b1fd47eaeec0e2d7ec5ce4cc77ac695f";

pub fn read_input(year: usize, day: usize) -> String {
    let path = format!("data/{year}/{day:02}.txt");
    let input = std::fs::read_to_string(&path);

    match input {
        Ok(raw) => raw,
        Err(_) => {
            let raw = download_input(TOKEN, year, day);
            std::fs::create_dir_all(format!("data/{year}/"))
                .expect(format!("could not create directory for {year}").as_str());
            let mut file =
                File::create(&path).expect(format!("Could not create File {year}-{day}").as_str());
            file.write_all(&raw.bytes().collect::<Vec<u8>>())
                .expect(format!("Could not write data to file {year}-{day}").as_str());
            raw
        }
    }
    .to_string()
}

fn download_input(token: &str, year: usize, day: usize) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let mut response = client
        .get(url)
        .header("Cookie", format!("session={token}"))
        .send()
        .unwrap()
        .text()
        .expect(format!("Could not download input {year}-{day}").as_str());
    if response.starts_with("Puzzle inputs differ by user.") {
        panic!("Session token invalid.");
    } else if response
        .starts_with("Please don't repeatedly request this endpoint before it unlocks!")
    {
        panic!("Input not yet available.");
    }
    if !response.ends_with('\n') {
        response.push('\n');
    }
    response
}

pub fn read_test_input(year: usize, day: usize, test: &str) -> String {
    let path = format!("data/{year}/tests/{day:02}.{test}.txt");
    std::fs::read_to_string(&path).expect(format!("{path} not found").as_str())
}
