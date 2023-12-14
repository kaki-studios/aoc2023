//NOTE: WILL NOT WORK IN REASONABLE AMOUNT OF TIME!

use std::{i32, usize};

fn main() {
    let test_input1 = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    // let test_springs = vec![
    //     SpringStatus::Operational,
    //     SpringStatus::Operational,
    //     SpringStatus::Damaged,
    //     SpringStatus::Operational,
    // ];
    //
    // let test_matches = matches(&test_springs, &vec![2, 1]);
    // assert!(test_matches);
    let result1 = answer(test_input1);
    assert_eq!(result1, 525152);
    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

fn answer(input: &str) -> i64 {
    let springs_str: Vec<&str> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap().0)
        .collect();
    let lists_str: Vec<&str> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap().1)
        .collect();
    let mut lists_vec: Vec<Vec<i32>> = lists_str
        .iter()
        .map(|line| {
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut springs_vec: Vec<Vec<SpringStatus>> = springs_str
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
    lists_vec = unfold_lists(lists_vec);
    springs_vec = unfold_springs(springs_vec);
    let mut answer: i64 = 0;
    for (i, (springs, lists)) in springs_vec.iter().zip(lists_vec).enumerate() {
        //heres the main juice of the program
        let num_of_unknowns = springs
            .iter()
            .filter(|status| **status == SpringStatus::Unknown)
            .count();
        let max = 2_i64.pow(num_of_unknowns as u32);
        dbg!(max);
        for curr in 0..max {
            println!("-------------------------");
            println!("{}", curr as f32 / max as f32);
            println!("of");
            println!("{}", i as f32 / springs.clone().len() as f32);
            println!("answers: {}", answer);
            println!("-------------------------");
            print!("\x1B[2J\x1B[1;1H"); //clears screen
            let mut curr_state = format!("{:b}", curr).chars().rev().collect::<Vec<char>>();
            if curr_state.len() < num_of_unknowns {
                curr_state.extend("0".repeat(num_of_unknowns - curr_state.len()).chars());
            }
            let filled_springs: Vec<SpringStatus> = springs
                .iter()
                .map(|status| match status {
                    SpringStatus::Unknown => match curr_state.pop().unwrap() {
                        '1' => SpringStatus::Operational,
                        '0' => SpringStatus::Damaged,
                        other => panic!("invalid digit: {}", other),
                    },
                    other => *other,
                })
                .collect();
            // dbg!(filled_springs.clone());
            if matches(&filled_springs, &lists) {
                dbg!(filled_springs.clone(), curr_state.clone());
                answer += 1;
            }
        }
    }

    answer
}

fn matches(input: &Vec<SpringStatus>, target: &Vec<i32>) -> bool {
    let damages = get_ranges(&input, SpringStatus::Damaged);
    if damages.len() != target.len() {
        return false;
    }
    for (spring, target) in damages.iter().zip(target) {
        // dbg!(spring.clone(), *target);
        if spring.clone().len() as i32 != *target {
            return false;
        }
    }
    true
}

fn get_ranges(springs: &Vec<SpringStatus>, status: SpringStatus) -> Vec<Vec<usize>> {
    let spring_ranges_ids = get_springs(springs, status);
    let mut slice_start = 0;
    let mut result_slice = Vec::new();
    for i in 1..spring_ranges_ids.len() {
        if spring_ranges_ids[i - 1] + 1 != spring_ranges_ids[i] {
            result_slice.push(spring_ranges_ids[slice_start..i].to_vec());
            slice_start = i;
        }
    }
    if spring_ranges_ids.len() > 0 {
        result_slice.push(spring_ranges_ids[slice_start..].to_vec());
    }
    result_slice.to_vec()
}
fn get_springs(springs: &Vec<SpringStatus>, status: SpringStatus) -> Vec<usize> {
    let mut spring_ids: Vec<usize> = vec![];
    for (i, spring) in springs.iter().enumerate() {
        if spring == &status {
            spring_ids.push(i);
        }
    }
    // dbg!(spring_ranges_ids.len() % 2 == 0);
    spring_ids
}

fn unfold_springs(mut springs_vec: Vec<Vec<SpringStatus>>) -> Vec<Vec<SpringStatus>> {
    for springs in springs_vec.iter_mut() {
        springs.push(SpringStatus::Unknown);
        *springs = springs.repeat(5);
        springs.pop();
    }
    springs_vec
}

fn unfold_lists(mut lists_vec: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for list in lists_vec.iter_mut() {
        *list = list.repeat(5);
    }
    lists_vec
}
