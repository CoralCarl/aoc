pub mod structure;

pub mod parsing {
    use std::fs::File;
    use std::io::prelude::*;
    const TOKEN: &str = "53616c7465645f5f4029b94729f7f6d71b2c9623504850cb36ec9c714bea441f2e450f5534b06844bc3efefb9eb33d15a72ea49609d95e0784d6b6f4a0828db0";

    fn download_input(token: &str, year: usize, day: usize) -> String {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        client
            .get(url)
            .header("Cookie", format!("session={token}"))
            .send()
            .unwrap()
            .text()
            .expect(format!("Could not download input {year}-{day}").as_str())
    }

    pub fn get_string(year: usize, day: usize) -> String {
        let path = format!("data/{year}/{day}.txt");
        let input = std::fs::read_to_string(&path);
        match input {
            Ok(raw) => raw,
            Err(_) => {
                let raw = download_input(TOKEN, year, day);
                std::fs::create_dir_all(format!("data/{year}/")).expect(format!("could not create directory for {year}").as_str());
                let mut file = File::create(&path)
                    .expect(format!("Could not create File {year}-{day}").as_str());
                file.write_all(&raw.bytes().collect::<Vec<u8>>())
                    .expect(format!("Could not write data to file {year}-{day}").as_str());
                raw
            }
        }
        .trim_end()
        .to_string()
    }

    pub fn get_lines(year: usize, day: usize) -> Vec<String> {
        get_string(year, day)
            .lines()
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect()
    }

    pub fn get_numbers<T>(year: usize, day: usize, delimiter: &str) -> Vec<Vec<T>>
    where
        T: std::str::FromStr,
    {
        let mut numbers: Vec<Vec<T>> = Vec::new();
        for line in get_string(year, day).lines() {
            if line != "" {
                let line_num: Vec<T> = line
                    .split(delimiter)
                    .filter_map(|num| num.parse::<T>().ok())
                    .collect();
                numbers.push(line_num);
            }
        }
        numbers
    }
}
