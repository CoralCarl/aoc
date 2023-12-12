pub mod parse {
    use std::str::FromStr;
    use std::fmt::Debug;

    pub fn grid<T>(input: &str, delimiter: &str) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        input
            .lines()
            .map(|line| {
                line.split(delimiter)
                    .map(|c| c.parse::<T>().expect("could not parse token"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}
