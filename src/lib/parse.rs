pub fn grid(input: &str) -> Vec<Vec<char>>
{
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn grid_map<F, T>(input: &str, delimiter: &str, map: F) -> Vec<Vec<T>>
where
    F: Fn(&str) -> T,
{
    input
        .lines()
        .map(|line| {
            line.split(delimiter)
                .filter(|token| *token != "")
                .map(|token| map(token))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
