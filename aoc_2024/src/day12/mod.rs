use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    Grid::new(input).total_price(false)
}

pub fn part_two(input: &str) -> usize {
    Grid::new(input).total_price(true)
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

struct Grid {
    plots: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
    directions: Vec<Direction>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let (rows, columns) = (map.len(), map[0].len());

        Self {
            plots: map,
            rows,
            columns,
            directions: vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ],
        }
    }

    fn in_bound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.rows as i32 && y >= 0 && y < self.columns as i32
    }

    fn get_region(
        &self,
        x: i32,
        y: i32,
        plant: char,
        visited: &mut HashSet<(i32, i32)>,
    ) -> HashSet<(i32, i32)> {
        let mut region = HashSet::from([(x, y)]);
        visited.insert((x, y));

        for dir in &self.directions {
            let (dx, dy) = dir.get_delta();
            let (i, j) = (x + dx, y + dy);
            if !visited.contains(&(i, j))
                && self.in_bound(i, j)
                && self.plots[i as usize][j as usize] == plant
            {
                let p = self.get_region(i, j, plant, visited);
                region.extend(p);
            } else {
                continue;
            }
        }
        region
    }

    fn total_price(&self, sides: bool) -> usize {
        let mut visited = HashSet::new();
        let mut prices = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.columns {
                let (x, y) = (i as i32, j as i32);
                if !visited.contains(&(x, y)) {
                    let region = self.get_region(x, y, self.plots[i][j], &mut visited);
                    let perimeters = self.count_sides(&region);
                    let perimeter = if sides { perimeters[1] } else { perimeters[0] };

                    prices.push(region.len() * perimeter);
                }
            }
        }
        prices.iter().sum()
    }

    /// Count total sides in a Rectilinear polygon
    ///
    ///     1    2
    ///     ┌────┐
    ///     │RRRR│3
    ///     │RRRR└┐4
    ///  10 └─┐RRR│
    ///      9│R┌─┘5
    ///       └─┘6
    ///       8 7
    ///
    fn count_sides(&self, region: &HashSet<(i32, i32)>) -> Vec<usize> {
        let mut perimeters = Vec::from([0, 0]);

        self.directions.iter().for_each(|dir| {
            let (dx, dy) = dir.get_delta();
            let mut points = HashMap::new();

            region.iter().for_each(|p| {
                let neighbor = (p.0 + dx, p.1 + dy);
                if !region.contains(&neighbor) {
                    perimeters[0] += 1;

                    let (key, value) = match dir {
                        Direction::Up | Direction::Down => (p.0, *p),
                        Direction::Left | Direction::Right => (p.1, (p.1, p.0)),
                    };

                    points.entry(key).or_insert_with(Vec::new).push(value);
                }
            });

            for vec in points.values_mut() {
                vec.sort_by_key(|k| k.1);
                let mut prev = vec[0];
                perimeters[1] += 1;

                for p in &vec[1..] {
                    if (p.1 - prev.1) > 1 {
                        perimeters[1] += 1;
                    }
                    prev = *p;
                }
            }
        });

        perimeters
    }
}

crate::aoctest!(1930, 1206);
