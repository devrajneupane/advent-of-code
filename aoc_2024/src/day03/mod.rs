use regex::Regex;

pub fn part_one(input: &str) -> u32 {
    parse(input)
}

pub fn part_two(input: &str) -> u32 {
    input
        .split("do()")
        .filter_map(|block| block.split("don't()").next().map(parse))
        .sum()
}

fn parse(input: &str) -> u32 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    re.captures_iter(input)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum()
}

crate::aoctest!(161, 48);
