use regex::Regex;

#[cfg(test)]
static SPACE: (i32, i32) = (11, 7);

#[cfg(not(test))]
static SPACE: (i32, i32) = (101, 103);

pub fn part_one(input: &str) -> i32 {
    Position::new(input).solve(100).1
}

pub fn part_two(input: &str) -> i32 {
    Position::new(input).solve(SPACE.0 * SPACE.1).0
}

struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn new(p: (i32, i32), v: (i32, i32)) -> Self {
        Self { p, v }
    }

    fn navigate(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;

        // Teleport
        if self.p.0 > SPACE.0 - 1 {
            self.p.0 -= SPACE.0;
        } else if self.p.0 < 0 {
            self.p.0 += SPACE.0;
        }

        if self.p.1 > SPACE.1 - 1 {
            self.p.1 -= SPACE.1;
        } else if self.p.1 < 0 {
            self.p.1 += SPACE.1;
        }
    }
}

struct Position {
    robots: Vec<Robot>,
}

impl Position {
    fn new(input: &str) -> Self {
        // Regex pattern
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

        let robots = input
            .lines()
            .flat_map(|line| {
                re.captures_iter(line)
                    .map(|caps| {
                        let p = (
                            caps[1].parse::<i32>().unwrap(),
                            caps[2].parse::<i32>().unwrap(),
                        );
                        let v = (
                            caps[3].parse::<i32>().unwrap(),
                            caps[4].parse::<i32>().unwrap(),
                        );

                        Robot::new(p, v)
                    })
                    .next()
            })
            .collect();

        Self { robots }
    }

    fn solve(&mut self, seconds: i32) -> (i32, i32) {
        let mut result = (0, i32::MAX);
        (0..seconds).for_each(|i| {
            self.robots.iter_mut().for_each(|robot| {
                robot.navigate();
            });
            // self.debug();
            // println!("{}", i);
            let mut robot_counts = [0, 0, 0, 0];
            let x_mid = SPACE.0 / 2;
            let y_mid = SPACE.1 / 2;

            self.robots.iter().for_each(|robot| {
                // count the number of robots in each quadrant
                if robot.p.0 > x_mid && robot.p.1 < y_mid {
                    // Quadrant I
                    robot_counts[0] += 1;
                } else if robot.p.0 > x_mid && robot.p.1 > y_mid {
                    // Quadrant II
                    robot_counts[1] += 1;
                } else if robot.p.0 < x_mid && robot.p.1 > y_mid {
                    // Quadrant III
                    robot_counts[2] += 1;
                } else if robot.p.0 < x_mid && robot.p.1 < y_mid {
                    // Quadrant IV
                    robot_counts[3] += 1;
                }
            });

            let sf = robot_counts.iter().filter(|&x| *x > 0).product::<i32>();

            if seconds == SPACE.0 * SPACE.1 {
                if sf < result.1 {
                    result.1 = sf;
                    result.0 = i + 1;
                }
            } else {
                result.1 = sf;
            }
        });

        result
    }

    #[allow(dead_code)]
    fn debug(&self) {
        // I initially, found the answer by printing positions of robots after each `seconds` as
        // `#` and `seconds` that has easter egg: has longest consecutive `#`.

        let mut grid: [[i32; SPACE.0 as usize]; SPACE.1 as usize] =
            [[0; SPACE.0 as usize]; SPACE.1 as usize];

        self.robots.iter().for_each(|r| {
            grid[r.p.1 as usize][r.p.0 as usize] += 1;
        });

        grid.iter().for_each(|b| {
            println!(
                "{:?}",
                b.iter()
                    .map(|&x| if x == 0 { ' ' } else { '#' })
                    .collect::<String>()
            );
        });
    }
}

crate::aoctest!(12);
