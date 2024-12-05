use std::{cmp, collections::HashMap};

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn part_one(input: &str) -> u32 {
    let (first_section, second_section) = parse(input);

    second_section
        .iter()
        .filter(|&pages| is_correct_order(&first_section, pages))
        .map(|pages| pages[pages.len() / 2].parse::<u32>().unwrap())
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let (first_section, mut second_section) = parse(input);

    second_section
        .iter_mut()
        .filter(|pages| !is_correct_order(&first_section, pages))
        .map(|pages| {
            pages.sort_unstable_by(|a, b| {
                first_section
                    .get(a)
                    .map_or(cmp::Ordering::Greater, |pagess| {
                        if pagess.contains(b) {
                            cmp::Ordering::Less
                        } else {
                            cmp::Ordering::Greater
                        }
                    })
            });
            pages[pages.len() / 2].parse::<u32>().unwrap()
        })
        .sum()
}

fn is_correct_order(first_section: &Rules, pages: &Vec<&str>) -> bool {
    pages.iter().enumerate().all(|(idx, &page)| {
        pages[idx + 1..].iter().all(|&page_| {
            first_section
                .get(&page)
                .map_or(false, |pages| pages.contains(&page_))
        })
    })
}

fn parse(input: &str) -> (Rules, Vec<Vec<&str>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    if sections.len() != 2 {
        return (HashMap::new(), Vec::new());
    }

    let mut first_section: Rules = HashMap::new();
    for line in sections[0].lines() {
        if let Some((key, value)) = line.split_once("|") {
            first_section.entry(key).or_default().push(value);
        }
    }

    let second_section: Vec<Vec<&str>> = sections[1]
        .lines()
        .map(|line| line.split(',').collect())
        .collect();

    (first_section, second_section)
}

crate::aoctest!(143, 123);

// use std::{cmp, collections::HashMap};
//
// pub fn part_one(input: &str) -> u32 {
//     let manual = Manual::parse(input);
//     manual
//         .pages
//         .iter()
//         .filter(|&pages| manual.is_order_correct(pages))
//         .map(|pages| pages[pages.len() / 2].parse::<u32>().unwrap())
//         .sum()
// }
//
// pub fn part_two(input: &str) -> u32 {
//     let mut manual = Manual::parse(input);
//
//     manual
//         .pages
//         .iter_mut()
//         .filter(|pages| !manual.is_order_correct(pages))
//         .map(|pages| {
//             pages.sort_unstable_by(|a, b| {
//                 manual
//                     .rules
//                     .get(a)
//                     .map_or(cmp::Ordering::Greater, |pagess| {
//                         if pagess.contains(b) {
//                             cmp::Ordering::Less
//                         } else {
//                             cmp::Ordering::Greater
//                         }
//                     })
//             });
//             pages[pages.len() / 2].parse::<u32>().unwrap()
//         })
//         .sum()
// }
//
// struct Manual<'a> {
//     rules: HashMap<&'a str, Vec<&'a str>>,
//     pages: Vec<Vec<&'a str>>,
// }
//
// impl<'a> Manual<'a> {
//     fn parse(input: &'a str) -> Self {
//         let sections: Vec<&str> = input.split("\n\n").collect();
//
//         if sections.len() != 2 {
//             return Self {
//                 rules: HashMap::new(),
//                 pages: Vec::new(),
//             };
//         }
//
//         let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
//         for line in sections[0].lines() {
//             if let Some((key, value)) = line.split_once("|") {
//                 rules.entry(key).or_default().push(value);
//             }
//         }
//
//         let pages: Vec<Vec<&str>> = sections[1]
//             .lines()
//             .map(|line| line.split(',').collect())
//             .collect();
//
//         Self { rules, pages }
//     }
//
//     fn is_order_correct(&self, pages: &Vec<&str>) -> bool {
//         pages.iter().enumerate().all(|(idx, &page)| {
//             pages[idx + 1..].iter().all(|&page_| {
//                 self.rules
//                     .get(&page)
//                     .map_or(false, |pages| pages.contains(&page_))
//             })
//         })
//     }
// }
//
// crate::aoctest!(143, 123);
