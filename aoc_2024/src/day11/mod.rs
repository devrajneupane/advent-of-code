use std::collections::HashMap;

static PART_ONE_BLINK_COUNT: u8 = 25;
static PART_TWO_BLINK_COUNT: u8 = 75;

pub fn part_one(input: &str) -> usize {
    let mut memoized = HashMap::new();
    parse(input)
        .map(|num| blink(num, PART_ONE_BLINK_COUNT, &mut memoized))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut memoized = HashMap::new();
    parse(input)
        .map(|num| blink(num, PART_TWO_BLINK_COUNT, &mut memoized))
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = u64> + use<'_> {
    input.split_whitespace().flat_map(str::parse)
}

fn blink(num: u64, blink_count: u8, memoized: &mut HashMap<(u64, u8), usize>) -> usize {
    if blink_count == 0 {
        return 1;
    }

    if let Some(res) = memoized.get(&(num, blink_count)) {
        return *res;
    }

    let count = match num {
        0 => blink(1, blink_count - 1, memoized),
        n if num.ilog10() % 2 == 1 => {
            let operand = 10u64.pow(num.ilog10() / 2 + 1);
            blink(n / operand, blink_count - 1, memoized)
                + blink(n % operand, blink_count - 1, memoized)
        }
        n => blink(n * 2024, blink_count - 1, memoized),
    };
    memoized.insert((num, blink_count), count);
    count
}

crate::aoctest!(55312);
