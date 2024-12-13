use std::collections::HashMap;

use regex::Regex;

pub fn part_one(input: &str) -> i64 {
    Machine::new(input).solve(0_i64)
}

pub fn part_two(input: &str) -> i64 {
    Machine::new(input).solve(10000000000000_i64)
}

// #[derive(Debug)]
// struct Matrix2x2 {
//     data: [[f64; 2]; 2],
// }
//
// impl Matrix2x2 {
//     fn new(data: [[f64; 2]; 2]) -> Self {
//         Self { data }
//     }
//
//     fn determinant(&self) -> f64 {
//         self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
//     }
//
//     fn inverse(&self) -> Option<Self> {
//         let det = self.determinant();
//
//         if det == 0.0 {
//             return None;
//         }
//
//         let inv_det = 1_f64 / det;
//         let inverse_matrix = [
//             [self.data[1][1] * inv_det, -self.data[0][1] * inv_det],
//             [-self.data[1][0] * inv_det, self.data[0][0] * inv_det],
//         ];
//         Some(Self::new(inverse_matrix))
//     }
//
//     fn mul_vec(&self, vector: &[f64; 2]) -> [f64; 2] {
//         [
//             self.data[0][0] * vector[0] + self.data[0][1] * vector[1],
//             self.data[1][0] * vector[0] + self.data[1][1] * vector[1],
//         ]
//     }
// }
//
// struct MachineConf {
//     config: Vec<(Matrix2x2, [f64; 2])>,
// }
//
// impl MachineConf {
//     fn new(input: &str) -> Self {
//         // Regex patterns
//         let button = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
//         let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
//         let config = input
//             .split("\n\n")
//             .map(|machine| {
//                 let mut matrix = [[0.0, 0.0], [0.0, 0.0]];
//                 let mut vector: [f64; 2] = [0.0, 0.0];
//                 button
//                     .captures_iter(machine)
//                     .enumerate()
//                     .for_each(|(i, caps)| {
//                         let _button = caps[1].chars().next().unwrap();
//                         matrix[0][i] = caps[2].parse::<f64>().unwrap();
//                         matrix[1][i] = caps[3].parse::<f64>().unwrap();
//                     });
//
//                 prize.captures_iter(machine).for_each(|caps| {
//                     vector = [
//                         caps[1].parse::<f64>().unwrap(),
//                         caps[2].parse::<f64>().unwrap(),
//                     ];
//                 });
//                 (Matrix2x2::new(matrix), vector)
//             })
//             .collect();
//
//         Self { config }
//     }
//
//     fn solve(&self) -> u64 {
//         self.config
//             .iter()
//             .map(|(mat, vec)| {
//                 if let Some(inv_mat) = mat.inverse() {
//                     let res = inv_mat.mul_vec(vec);
//                     if [
//                         mat.data[0][0] * res[0] + mat.data[0][1] * res[1],
//                         mat.data[1][0] * res[0] + mat.data[1][1] * res[1],
//                     ] == *vec
//                     {
//                         if res.iter().all(|&v| v <= 100) {
//                             // if res.iter().all(|&v|  v <= 100) {
//                             res[0] * 3 + res[1]
//                         } else {
//                             0.0
//                         }
//                     } else {
//                         0.0
//                     }
//                     // if res.iter().all(|&v| v >= 0.0 && v <= 100) {
//                     //     // if res.iter().all(|&v|  v <= 100) {
//                     //     res[0] * 3 + res[1]
//                     // } else {
//                     //     0.0
//                     // }
//                 } else {
//                     0.0
//                 }
//             })
//             .sum::<f64>() as u64
//     }
// }

#[derive(Debug)]
struct Machine {
    machines: Vec<HashMap<char, (i64, i64)>>,
}

impl Machine {
    fn new(input: &str) -> Self {
        // Regex patterns
        let button = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
        let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let m: Vec<HashMap<char, (i64, i64)>> = input
            .split("\n\n")
            .map(|machine| {
                let mut map = HashMap::new();
                button.captures_iter(machine).for_each(|caps| {
                    let button = caps[1].chars().next().unwrap();
                    map.insert(
                        button,
                        (
                            caps[2].parse::<i64>().unwrap(),
                            caps[3].parse::<i64>().unwrap(),
                        ),
                    );
                });

                prize.captures_iter(machine).for_each(|caps| {
                    map.insert(
                        'P',
                        (
                            caps[1].parse::<i64>().unwrap(),
                            caps[2].parse::<i64>().unwrap(),
                        ),
                    );
                });
                map
            })
            .collect();
        Self { machines: m }
    }

    /// I got stuck today with floating point operations while implementing Cramer's rule
    /// (https://en.wikipedia.org/wiki/Cramer%27s_rule). test case works fine with my implmentation
    /// but actual input gives higher result. so i followed following thread, which use same method
    /// with simplified equtions.
    /// https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
    ///
    fn solve(&self, offset: i64) -> i64 {
        self.machines
            .iter()
            .map(|machine| {
                let prize = (machine[&'P'].0 + offset, machine[&'P'].1 + offset);
                let (m_a, m_b) = (machine[&'A'], machine[&'B']);

                let det = m_a.0 * m_b.1 - m_a.1 * m_b.0;
                let a = (prize.0 * m_b.1 - prize.1 * m_b.0) / det;
                let b = (m_a.0 * prize.1 - m_a.1 * prize.0) / det;

                if (m_a.0 * a + m_b.0 * b, m_a.1 * a + m_b.1 * b) == (prize.0, prize.1) {
                    a * 3 + b
                } else {
                    0
                }
            })
            .sum()
    }
}

crate::aoctest!(480);
