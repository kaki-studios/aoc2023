use core::panic;
use std::{collections::BTreeMap, u32};

fn main() {
    let test_input1 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    let result1 = answer(test_input1);
    assert_eq!(result1, 6);

    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> usize {
    let instructions: Vec<char> = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    println!("{:?}", instructions);

    let nodes_str_raw: String = input.split_once("\n\n").unwrap().1.to_string();
    let mut node_map: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    for line in nodes_str_raw.lines() {
        let id = line.split_once(" = ").unwrap().0;
        let left = &line
            .split_once(" = ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap()
            .0[1..];
        let right = &line
            .split_once(" = ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap()
            .1
            .trim_end()[..3];

        node_map.insert(id, (left, right));
    }

    println!("{:?}", node_map);
    let curr_nodes: Vec<&str> = node_map
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();
    println!("{}", curr_nodes.len());
    let cycles = curr_nodes
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;
            // cycle is magically "the first Z",
            // and other assorted conditions due
            // to how the input is constructed.
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = node_map
                        .get(current_node)
                        .expect("always exist at a valid node");
                    let next_node = match instruction {
                        'L' => options.0,
                        'R' => options.1,
                        other => panic!("invalid instruction: {other}"),
                    };
                    if next_node.ends_with("Z") {
                        Some(index + 1)
                    } else {
                        visited_nodes.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a cycle")
        })
        .collect::<Vec<usize>>();
    println!("{:?}", cycles);
    let result = lcm(&cycles);
    result
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
