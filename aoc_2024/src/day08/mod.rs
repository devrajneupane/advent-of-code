use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

pub fn part_one(input: &str) -> usize {
    Antenna::new(input, false).count_antinodes()
}

pub fn part_two(input: &str) -> usize {
    Antenna::new(input, true).count_antinodes()
}

struct Antenna {
    map: HashMap<char, Vec<Point>>,
    rows: usize,
    columns: usize,
    resonant_effect: bool,
}

impl Antenna {
    fn new(input: &str, resonant_effect: bool) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let map = lines
            .iter()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().filter_map(move |(y, freq)| {
                    (freq != '.').then_some((
                        freq,
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                    ))
                })
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<char, Vec<Point>>, (freq, p)| {
                    acc.entry(freq).or_default().push(p);
                    acc
                },
            );

        Self {
            map,
            rows: lines.len(),
            columns: lines[0].len(),
            resonant_effect,
        }
    }

    fn in_bound(&self, p: &Point) -> bool {
        (0..self.rows as i32).contains(&p.x) && (0..self.columns as i32).contains(&p.y)
    }

    fn count_antinodes(&self) -> usize {
        self.map
            .values()
            .flat_map(|points| {
                points.iter().enumerate().flat_map(|(i, p1)| {
                    points[i + 1..].iter().flat_map(|p2| p1.antinodes(p2, self))
                })
            })
            .collect::<HashSet<Point>>()
            .len()
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn antinodes(&self, other: &Point, antenna: &Antenna) -> Vec<Self> {
        let (dx, dy) = ((other.x - self.x), (other.y - self.y));

        if !antenna.resonant_effect {
            return [self.offset(-dx, -dy), other.offset(dx, dy)]
                .into_iter()
                .filter(|p| antenna.in_bound(p))
                .collect();
        }

        successors(Some(self.clone()), |p| {
            let next = p.offset(-dx, -dy);
            antenna.in_bound(&next).then_some(next)
        })
        .chain(successors(Some(other.clone()), |p| {
            let next = p.offset(dx, dy);
            antenna.in_bound(&next).then_some(next)
        }))
        .collect()
    }
}

crate::aoctest!(14, 34);
