pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|report| is_safe_report(report, None))
        .count()
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|&report| {
            is_safe_report(report, None)
                || (0..report.len()).any(|i| is_safe_report(report, Some(i)))
        })
        .count()
}

fn is_safe_report(report: &str, skip_index: Option<usize>) -> bool {
    let vec: Vec<u32> = report
        .split_whitespace()
        .enumerate()
        .filter_map(|(index, level)| {
            if Some(index) == skip_index {
                None
            } else {
                level.parse().ok()
            }
        })
        .collect();

    // Check ascending sequence
    let asc_vec = vec
        .windows(2)
        .all(|w| (w[0] < w[1] && (1..=3).contains(&(w[1] - w[0]))));

    // Check descending sequence
    let dsc_vec = vec
        .windows(2)
        .all(|w| (w[0] > w[1] && (1..=3).contains(&(w[0] - w[1]))));

    asc_vec || dsc_vec
}

crate::aoctest!(2, 4);
