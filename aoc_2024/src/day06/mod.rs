use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut map = Map::parse(input);
    let mut set = HashSet::new();
    set.insert(map.pos);

    while (1..map.grid.len() - 1).contains(&map.pos.0)
        && (1..map.grid[0].len() - 1).contains(&map.pos.1)
    {
        map.try_move();
        set.insert(map.pos);
    }
    set.len()
}

pub fn part_two(input: &str) -> usize {
    let mut map = Map::parse(input);
    // let mut count = 0;
    let count = 0;
    let mut set = HashSet::new();

    while (1..map.grid.len() - 1).contains(&map.pos.0)
        && (1..map.grid[0].len() - 1).contains(&map.pos.1)
    {
        map.try_move2();
    }
    set.insert(map.pos);

    count
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    pos: (usize, usize),
    dir: Direction,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut pos = (0, 0);
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find("^") {
                    pos = (i, j);
                }
                line.chars().collect()
            })
            .collect();
        Self {
            grid,
            pos,
            dir: Direction::Up,
        }
    }

    fn try_move(&mut self) {
        let (dx, dy) = match self.dir {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        let new_pos = (
            (self.pos.0 as i32 + dx) as usize,
            (self.pos.1 as i32 + dy) as usize,
        );

        if self.grid[new_pos.0][new_pos.1] == '#' {
            self.dir = self.dir.next();
        } else {
            self.pos = new_pos;
        }
    }

    fn try_move2(&mut self) {
        let (dx, dy) = match self.dir {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        let new_pos = (
            (self.pos.0 as i32 + dx) as usize,
            (self.pos.1 as i32 + dy) as usize,
        );

        if self.grid[new_pos.0][new_pos.1] == '#' {
            self.dir = self.dir.next();
        } else {
            self.grid[new_pos.0][new_pos.1] = '#';
            self.try_move();
            // check if loop is occured
            self.grid[new_pos.0][new_pos.1] = '.';
            self.pos = new_pos;
        }
    }
}

crate::aoctest!(41, 6);
