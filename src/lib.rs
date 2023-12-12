pub mod structure;

pub mod parsing {
    use std::fs::File;
    use std::io::prelude::*;
    const TOKEN: &str = "53616c7465645f5f572d38c54040f242a65b98b4bdbd09b842cb33c29bf7a4838095a854b3699265d56d731a716a2ed6b1fd47eaeec0e2d7ec5ce4cc77ac695f";

    fn download_input(token: &str, year: usize, day: usize) -> String {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let response = client
            .get(url)
            .header("Cookie", format!("session={token}"))
            .send()
            .unwrap()
            .text()
            .expect(format!("Could not download input {year}-{day}").as_str());
        if response == "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" {
            panic!("Session token invalid.");
        }
        response
    }

    pub fn get_string(year: usize, day: usize) -> String {
        let path = format!("data/{year}/{day}.txt");
        let input = std::fs::read_to_string(&path);
        match input {
            Ok(raw) => raw,
            Err(_) => {
                let raw = download_input(TOKEN, year, day);
                std::fs::create_dir_all(format!("data/{year}/"))
                    .expect(format!("could not create directory for {year}").as_str());
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
            .map(|x| x.to_string())
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

pub mod geometry {
    use std::ops::{Add, AddAssign, Sub, SubAssign};
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T> {
        pub fn distance(&self, other: Point<T>) -> usize
        where
            T: std::ops::Sub<Output = usize>,
            T: Copy + PartialOrd,
        {
            let y = if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            };
            let x = if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            };
            
            (x + y) as usize
        }
    }

    impl<T: Add<Output = T>> Add for Point<T> {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl<T: AddAssign<T>> AddAssign for Point<T> {
        fn add_assign(&mut self, other: Self) {
            self.x += other.x;
            self.y += other.y;
        }
    }

    impl<T: Sub<Output = T>> Sub for Point<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl<T: SubAssign<T>> SubAssign for Point<T> {
        fn sub_assign(&mut self, other: Self) {
            self.x -= other.x;
            self.y -= other.y;
        }
    }
}
