mod day01;
mod day02;
mod day03;

/// Run solution for specific problem based on `day` and `part`
///
/// * `day`: problem day
/// * `part`: part of the problem
///   TODO: How to convert this into macro?
pub fn get_solution(day: &str, part: &str) -> Result<String, String> {
    let input_path = format!("src/day{}/input.txt", day);
    let input = std::fs::read_to_string(input_path).expect("File not found");

    match (day, part) {
        ("01", "1") => Ok(day01::part_one(&input).to_string()),
        ("01", "2") => Ok(day01::part_two(&input).to_string()),
        ("02", "1") => Ok(day02::part_one(&input).to_string()),
        ("02", "2") => Ok(day02::part_two(&input).to_string()),
        ("03", "1") => Ok(day03::part_one(&input).to_string()),
        ("03", "2") => Ok(day03::part_two(&input).to_string()),
        _ => Err(format!("Invalid day or part: {} - {}", day, part)),
    }
}

#[macro_export]
macro_rules! aoctest {
    ($part_one_test:expr) => {
        #[cfg(test)]
        mod tests {
            static TEST_INPUT: &str = include_str!("example.txt");

            #[test]
            fn part_one_test() {
                let output = super::part_one(TEST_INPUT);
                assert_eq!(output, $part_one_test);
            }
        }
    };

    ($part_one_test:expr, $part_two_test:expr) => {
        #[cfg(test)]
        mod tests {
            static TEST_INPUT: &str = include_str!("example.txt");

            #[test]
            fn part_one_test() {
                let output = super::part_one(TEST_INPUT);
                assert_eq!(output, $part_one_test);
            }

            #[test]
            fn part_two_test() {
                let output = super::part_two(TEST_INPUT);
                assert_eq!(output, $part_two_test);
            }
        }
    };
}
