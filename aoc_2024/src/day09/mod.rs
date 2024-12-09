pub fn part_one(input: &str) -> usize {
    let mut map = DiskMap::new(input);
    map.rearrange()
}

pub fn part_two(input: &str) -> usize {
    let mut map = DiskMap::new(input);
    map.vec_rearrange();
    map.update_checksum()
}

#[derive(Debug)]
struct DiskMap {
    map: Vec<Vec<String>>,
}

impl DiskMap {
    fn new(input: &str) -> Self {
        let mut block_id = -1;
        Self {
            map: input
                .trim_end()
                .chars()
                .enumerate()
                .fold(Vec::new(), |mut acc, (i, ch)| {
                    let mut count = ch.to_digit(10).unwrap() as usize;
                    if i % 2 == 0 {
                        block_id += 1;
                        let mut tmp = Vec::new();
                        while count != 0 {
                            tmp.push(block_id.to_string());
                            count -= 1
                        }
                        acc.push(tmp);
                        acc
                    } else {
                        let mut tmp = Vec::new();
                        while count != 0 {
                            tmp.push(".".to_string());
                            count -= 1
                        }
                        if !tmp.is_empty() {
                            acc.push(tmp);
                        }
                        acc
                    }
                }),
        }
    }

    fn vec_rearrange(&mut self) {
        let (mut start, end) = (0, self.map.len() - 1);

        for i in (1..=end).rev() {
            let (start_vecc, end_vecc) = self.map.split_at_mut(i);
            let end_vec = &mut end_vecc[0];

            if end_vec.iter().all(|x| x != ".") {
                while start < i {
                    let start_vec = &mut start_vecc[start];
                    if start_vec.iter().all(|x| x == ".") && start_vec.len() >= end_vec.len() {
                        let swap_length = start_vec.len().min(end_vec.len());

                        let extra_elements: Vec<_> = if start_vec.len() > end_vec.len() {
                            start_vec[swap_length..].to_vec()
                        } else {
                            Vec::new()
                        };

                        // Take elements from both vectors
                        let first_elements: Vec<_> = start_vec[..swap_length].to_vec();
                        let last_elements: Vec<_> = end_vec[..swap_length].to_vec();

                        start_vec.clear();
                        end_vec.clear();

                        start_vec.extend(last_elements.into_iter().rev());
                        end_vec.extend(first_elements.into_iter().rev());

                        if !extra_elements.is_empty() {
                            self.map.insert(start + 1, extra_elements);
                        }

                        break;
                    }
                    start += 1;
                }
                start = 0;
            }
        }
    }

    fn update_checksum(&self) -> usize {
        self.map
            .concat()
            .iter()
            .enumerate()
            .fold(0, |mut acc, (i, file_id)| {
                if file_id != "." {
                    acc += i * file_id.parse::<usize>().unwrap();
                }
                acc
            })
    }

    fn rearrange(&mut self) -> usize {
        let tmp = &mut self.map.concat();
        let (mut start, mut end) = (0, tmp.len() - 1);

        while start < end {
            if tmp[start] == "." {
                if tmp[end] != "." {
                    tmp.swap(start, end);
                    start += 1;
                }
                end -= 1;
            } else {
                start += 1;
            }
        }
        tmp[..=end].iter().enumerate().fold(0, |acc, (i, file_id)| {
            acc + (i * file_id.parse::<usize>().unwrap())
        })
    }
}

crate::aoctest!(1928, 2858);
