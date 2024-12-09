use std::collections::VecDeque;

pub fn part_one(input: &str) -> usize {
    let map = Map::new(input);

    let mut all_paths = Vec::new();

    for row in 0..map.height {
        for col in 0..map.width {
            if map.map[row][col] == 0 {
                let paths = find_paths(&map, Pos { row, col });
                all_paths.push(paths.len());
            }
        }
    }

    all_paths.iter().sum()
}

pub fn part_two(_input: &str) -> u32 {
    todo!();
}

fn find_paths(map: &Map, start: Pos) -> Vec<Vec<Pos>> {
    let mut paths = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, vec![start], 0));

    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((current, path, value)) = queue.pop_front() {
        if map.map[current.row][current.col] == 9 {
            paths.push(path);
            continue;
        }

        for (dx, dy) in directions.iter() {
            let row = current.row as isize + dx;
            let col = current.col as isize + dy;

            if row >= 0 && row < map.height as isize && col >= 0 && col < map.width as isize {
                let new_pos = Pos {
                    row: row as usize,
                    col: col as usize,
                };
                let new_value = map.map[row as usize][col as usize];

                if new_value == value + 1 {
                    let mut new_path = path.clone();
                    new_path.push(new_pos);
                    queue.push_back((new_pos, new_path, new_value));
                }
            }
        }
    }
    paths
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect())
            .collect();
        let (height, width) = (map.len(), map[0].len());
        Self { map, height, width }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

crate::aoctest!(36);
