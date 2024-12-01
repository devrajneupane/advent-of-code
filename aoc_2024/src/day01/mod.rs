pub fn part_one(input: &str) -> u32 {
    let (mut vec1, mut vec2) = parse(input);

    // Sort inplace
    vec1.sort_unstable();
    vec2.sort_unstable();

    vec1.iter()
        .zip(vec2.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let (vec1, vec2) = parse(input);
    vec1.iter()
        .map(|num| {
            let count = vec2.iter().filter(|x| *x == num).count() as u32;
            num * count
        })
        .sum()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    // TODO: Refactor this
    input
        .lines()
        .filter_map(|line| {
            let mut ids = line.split_whitespace();
            match (ids.next(), ids.next()) {
                (Some(first), Some(second)) => {
                    match (first.parse::<u32>(), second.parse::<u32>()) {
                        (Ok(x), Ok(y)) => Some((x, y)),
                        _ => None,
                    }
                }
                _ => None,
            }
        })
        .unzip()
}

crate::aoctest!(11, 31);
