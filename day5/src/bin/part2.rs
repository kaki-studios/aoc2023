// use indicatif::ParallelProgressIterator;
use rayon::{iter::plumbing::UnindexedProducer, prelude::*};
use std::{i64, ops::Range};

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
    let map: Vec<Line> = vec![
        Line {
            range: 0..5,
            delta: 5,
        },
        Line {
            range: 10..12,
            delta: 2,
        },
    ];
    let seed_range: Range<i64> = 0..20;
    let split_test = split(&seed_range, map);
    println!(
        "split seeds test: {:?} \nshould print [10..15, 5..10, 12..14, 14..20]",
        split_test
    );
    //doesnt work

    //does not correctly map if seed isnt in map (should return the original)
    //46
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
    range: Range<i64>,
    delta: i64,
}

fn answer(input: &str) -> i64 {
    let seed_ranges_raw: Vec<i64> = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    let seed_ranges: Vec<Range<i64>> = seed_ranges_raw
        .chunks(2)
        .enumerate()
        .map(|range| range.1[0]..range.1[0] + range.1[1])
        .collect();

    // println!("done!");
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
                .map(|string| string.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            assert_eq!(3, nums.len());
            let num_line = Line {
                delta: nums[0] - nums[1],
                range: nums[1]..nums[1] + nums[2],
            };
            maps[i - 1].push(num_line);
        }
    }
    maps.iter_mut()
        .for_each(|map| map.sort_by_key(|r| r.range.start));

    let mut location = 1_i64;
    loop {
        let mut cur = location;
        for map in maps.iter().rev() {
            cur = reverse_lookup(map, cur);
        }
        for sr in &seed_ranges {
            if sr.contains(&cur) {
                return location;
            }
        }
        location += 1;
    }

    // process(seed_ranges, maps)
    //     .iter()
    //     .map(|range| range.start)
    //     .min()
    //     .unwrap()
}

fn reverse_lookup(map: &Vec<Line>, val: i64) -> i64 {
    for map in map {
        let rev = val - map.delta;
        if map.range.contains(&rev) {
            return rev;
        }
    }

    val
}

fn process(seed_ranges: Vec<Range<i64>>, maps: Vec<Vec<Line>>) -> Vec<Range<i64>> {
    // let mut answer: Vec<Range<i64>> = vec![];
    maps.iter()
        .map(|map| {
            let mut answer: Vec<Range<i64>> = vec![];
            seed_ranges.iter().for_each(|seed_range| {
                let intersections = split(seed_range, map.to_vec());
                if intersections.len() > 0 {
                    // println!("intersections exist!{}", intersections.len());
                    intersections.iter().for_each(|intersection| {
                        // println!("got to intersections iter");
                        answer.push(intersection.clone());
                        // println!(" intersection: here: {:?}", intersection);
                    });
                };
            });
            // println!("answer exists: {}", answer.len());
            answer
        })
        .collect::<Vec<Vec<Range<i64>>>>()
        .iter()
        .fold(vec![], |mut acc, range_vec| {
            range_vec.iter().for_each(|range| acc.push(range.clone()));
            // acc.append((range_vec));
            // println!("{}", acc.len());
            acc
        })
}
//like intersect but returns all intersections
fn split(seed_range: &Range<i64>, map: Vec<Line>) -> Vec<Range<i64>> {
    let mut result: Vec<Range<i64>> = vec![];
    let mut prev: Vec<Range<i64>> = vec![];
    map.iter().enumerate().for_each(|(i, line)| {
        let mut intersection = intersect(&line.range, seed_range);
        if i != 0 && prev.len() > 1 {
            intersection = prev.pop().unwrap();
        }

        if !intersection.is_empty() {
            if intersection.start == line.range.start {
                if !(intersection.end..line.range.end).is_empty() {
                    prev.push(intersection.end..line.range.end);
                    println!("true");
                }
            } else if intersection.end == line.range.end {
                if !(line.range.start..intersection.start).is_empty() {
                    prev.push(line.range.start..intersection.start);
                    println!("also true");
                }
            }
            //shift the intersection
            intersection.start += line.delta;
            intersection.end += line.delta;
            prev.push(intersection);
        }
    });
    result = prev;
    result
}

fn intersect(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
    max(a.start, b.start)..min(a.end, b.end)
}

fn min(a: i64, b: i64) -> i64 {
    if a > b {
        b
    } else {
        a
    }
}
fn max(a: i64, b: i64) -> i64 {
    if a < b {
        b
    } else {
        a
    }
}
// fn process(seed_ranges: Vec<Range<i64>>, maps: Vec<Vec<Line>>) -> Vec<Range<i64>> {
//     // let mut answer: Vec<Range<i64>> = vec![];
//     maps.iter()
//         .map(|map| {
//             let mut answer: Vec<Range<i64>> = vec![];
//             seed_ranges.iter().for_each(|seed_range| {
//                 let intersections = remap(seed_range, map);
//                 intersections.iter().for_each(|intersection| {
//                     answer.push(intersection.clone());
//                 });
//             });
//             answer
//         })
//         .collect::<Vec<Vec<Range<i64>>>>()
//         .iter()
//         .fold(vec![], |mut acc, range_vec| {
//             range_vec.iter().for_each(|range| acc.push(range.clone()));
//             acc
//         })
// }

// //maps the seed range through a map
// fn remap(seed_range: &Range<i64>, map: &Vec<Line>) -> Vec<Range<i64>> {
//     map.iter().enumerate().fold(vec![], |mut acc, (i, line)| {
//         if i == 0 {
//             acc = split(seed_range, &line.source_range);
//         }
//         let mut temp = acc.clone();
//         acc.iter()
//             .for_each(|range| temp = split(range, &line.source_range));
//         acc.clear();
//         acc = temp;
//         match acc.len() {
//             0 => {
//                 //0 intersections
//                 if i == map.len() - 1 {
//                     acc.push(seed_range.clone());
//                     return acc;
//                 }
//                 println!("shouldnt happen");
//             }
//             1 => {
//                 //the intersection is equal to a
//                 let maps_up = line.source_range.start < line.destination_range.start;
//                 let delta = if maps_up {
//                     line.destination_range.start - line.source_range.start
//                 } else {
//                     line.source_range.start - line.destination_range.start
//                 };
//                 if maps_up {
//                     //shift the intersection
//                     acc[0].start += delta;
//                     acc[0].end += delta;
//                 } else {
//                     acc[0].start -= delta;
//                     acc[0].end -= delta;
//                 }
//                 println!("shouldnt happen");
//             }
//             2 => {
//                 //normal
//                 //intersection is acc[1] and remainder is acc[0]
//                 println!("should happen");
//                 let maps_up = line.source_range.start < line.destination_range.start;
//                 let delta = if maps_up {
//                     line.destination_range.start - line.source_range.start
//                 } else {
//                     line.source_range.start - line.destination_range.start
//                 };
//                 if maps_up {
//                     //shift the intersection
//                     acc[1].start += delta;
//                     acc[1].end += delta;
//                 } else {
//                     acc[1].start -= delta;
//                     acc[1].end -= delta;
//                 }
//             }
//             other => println!("shouldnt happen at all, len is {other}"),
//         }
//         acc
//     })
// }
//
// //returns the intersection of a and b as the second item in the vec
//
// //and also the rest of a
// fn split(a: &Range<i64>, b: &Range<i64>) -> Vec<Range<i64>> {
//     let intersection_start = max(a.start, b.start);
//     let intersection_end = min(a.end, b.end);
//
//     let mut result: Vec<Range<i64>> = vec![];
//     if (intersection_start..intersection_end).is_empty() {
//         return result;
//     }
//     if intersection_start == a.start {
//         //a is bigger
//         if !(intersection_end..a.end).is_empty() {
//             result.push(intersection_end..a.end);
//         }
//         result.push(intersection_start..intersection_end);
//     } else {
//         //a is smaller
//         if !(a.start..intersection_start).is_empty() {
//             result.push(a.start..intersection_start);
//         }
//         result.push(intersection_start..intersection_end);
//     }
//     result
// }
