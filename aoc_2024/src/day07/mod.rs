use itertools::Itertools;

pub fn part_one(input: &str) -> u64 {
    Eq::new(input).calibrate("+*")
}

pub fn part_two(input: &str) -> u64 {
    Eq::new(input).calibrate("+*|")
}

struct Eq {
    exp: Vec<(u64, Vec<u64>)>,
}

impl Eq {
    fn new(input: &str) -> Self {
        Self {
            exp: input
                .lines()
                .filter_map(|line| line.split_once(':'))
                .map(|(result, numbers)| {
                    (
                        result.parse().unwrap(),
                        numbers
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    fn gen_op_seq(ops: &str, len: usize) -> Vec<String> {
        (0..len - 1)
            .map(|_| ops.chars())
            .multi_cartesian_product()
            .map(|v| v.into_iter().collect::<String>())
            .collect()
    }

    fn calibrate(&self, ops: &str) -> u64 {
        self.exp
            .iter()
            .filter_map(|(res, nums)| {
                Eq::gen_op_seq(ops, nums.len()).into_iter().find_map(|op| {
                    let result = op.chars().zip(nums.iter().skip(1)).fold(
                        nums[0],
                        |acc, (operator, &next_num)| match operator {
                            '+' => acc + next_num,
                            '*' => acc * next_num,
                            // _ => format!("{}{}", acc, next_num).parse().unwrap(),
                            _ => acc * 10u64.pow(next_num.ilog10() + 1) + next_num,
                        },
                    );

                    if result == *res {
                        Some(result)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }
}

crate::aoctest!(3749, 11387);
