use std::{collections::BTreeMap, u32};

fn main() {
    let test_input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let result1 = answer(test_input1);
    assert_eq!(result1, 2);
    println!("1st test success!");
    let test_input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let result2 = answer(test_input2);
    assert_eq!(result2, 6);
    println!("2nd test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> u32 {
    let mut result: u32 = 0;
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
    let mut curr_node: &str = "AAA";
    for character in instructions.iter().cycle() {
        if character == &'R' {
            curr_node = node_map.get(curr_node).unwrap().1;
            result += 1;
        } else if character == &'L' {
            curr_node = node_map.get(curr_node).unwrap().0;
            result += 1;
        }

        if curr_node == "ZZZ" {
            break;
        }
    }
    result
}
