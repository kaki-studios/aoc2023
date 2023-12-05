use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{ops::Range, u64};

fn main() {
    let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let result = answer(test_input);

    if result == 46 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

#[derive(Debug, Clone)]
struct Line {
    destination_range: Range<u64>,
    source_range: Range<u64>,
}

fn answer(input: &str) -> u64 {
    let seed_ranges_raw: Vec<u64> = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let seed_ranges: Vec<Range<u64>> = seed_ranges_raw
        .windows(2)
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|range| range.1[0]..range.1[0] + range.1[1])
        .collect();
    // println!("{:?}", seed_ranges);
    let mut seeds: Vec<u64> = seed_ranges
        .par_iter()
        .progress()
        .map(|range| range.clone().collect::<Vec<u64>>())
        .reduce(
            || vec![],
            |mut acc, range_vec| {
                acc.extend(range_vec);
                acc
            },
        )
        .into_iter()
        .collect();

    println!("done!");
    let mut maps: Vec<Vec<Line>> = vec![];
    // maps.push(vec![]);
    for (i, mut map_str) in input.split("\n\n").enumerate() {
        if i == 0 {
            continue;
            //we dont want to include the first line because it has the seeds
        }
        maps.push(vec![]);

        //map_str is the raw str format of the map
        map_str = map_str.split(":\n").last().unwrap();
        for line in map_str.lines() {
            let nums = line
                .split_whitespace()
                .map(|string| string.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            assert_eq!(3, nums.len());
            let num_line = Line {
                destination_range: nums[0]..nums[0] + nums[2],
                source_range: nums[1]..nums[1] + nums[2],
            };
            maps[i - 1].push(num_line);
        }
    }
    // println!("{:?}", maps);
    *seeds
        .par_iter_mut()
        .map(|seed| {
            'map: for map in &maps {
                for line in map {
                    if line.source_range.contains(&seed) {
                        *seed = (*seed - &line.source_range.start) + &line.destination_range.start;
                        continue 'map;
                    } else {
                        continue;
                    }
                }
            }
            *seed
        })
        .collect::<Vec<u64>>()
        .iter()
        .min()
        .unwrap()
}
