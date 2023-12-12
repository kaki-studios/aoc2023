use std::{i32, ops::Range};

fn main() {
    let test_input1 = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    let result1 = answer(test_input1);
    assert_eq!(result1, 21);
    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}
#[derive(Debug, PartialEq, Clone)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

fn answer(input: &str) -> i32 {
    let springs_str: Vec<&str> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap().0)
        .collect();
    let lists: Vec<&str> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap().1)
        .collect();
    let springs_vec: Vec<Vec<SpringStatus>> = springs_str
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => SpringStatus::Operational,
                    '#' => SpringStatus::Damaged,
                    '?' => SpringStatus::Unknown,
                    other => panic!("invalid char {}", other),
                })
                .collect()
        })
        .collect();
    dbg!(springs_vec.clone(), lists);

    for springs in springs_vec {
        let mut unknown_ranges_ids: Vec<usize> = vec![];
        for (i, spring_pair) in springs.windows(2).enumerate() {
            if i == 0 && spring_pair[0] == SpringStatus::Unknown {
                unknown_ranges_ids.push(i);
            }
            if (spring_pair[0] == SpringStatus::Operational
                || spring_pair[0] == SpringStatus::Damaged)
                && spring_pair[1] == SpringStatus::Unknown
            {
                unknown_ranges_ids.push(i + 1);
                continue;
            }
            if spring_pair[0] == SpringStatus::Unknown
                && (spring_pair[1] == SpringStatus::Operational
                    || spring_pair[1] == SpringStatus::Damaged)
            {
                unknown_ranges_ids.push(i + 1);
                continue;
            }
            if spring_pair[0] == SpringStatus::Unknown
                && spring_pair[1] == SpringStatus::Unknown
                && i == springs.len() - 2
            {
                unknown_ranges_ids.push(i + 2);
            }
        }
        let unknown_ranges = unknown_ranges_ids
            .chunks_exact(2)
            .map(|pair| pair[0]..pair[1])
            .collect::<Vec<Range<usize>>>();
        dbg!(unknown_ranges);
        //TODO: we have the ranges, nothing else, at 80 loc
        //this might take a while...
    }

    0
}
