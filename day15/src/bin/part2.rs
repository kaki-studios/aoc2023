use indexmap::*;
use std::{collections::BTreeMap, usize};

fn main() {
    let test_input1 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let result1 = answer(test_input1);
    assert_eq!(result1, 145);
    println!("Test success! Here\'s the answer:");
    let answer = answer(include_str!("../../input.txt"));
    println!("{}", answer);
}

fn answer(input: &str) -> u32 {
    let list = input.split(",");
    let mut boxes: Vec<IndexMap<String, u32>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(IndexMap::new());
    }
    // println!("{}", boxes.len());
    for string in list {
        let (label, command) = if string.contains("-") {
            (string.replace("-", ""), CommandType::Remove)
        } else {
            (
                string
                    .chars()
                    .take_while(|ch| ch != &'=')
                    .collect::<String>(),
                CommandType::Assign({
                    string
                        .chars()
                        .rev()
                        // .take_while(|ch| ch.is_digit(10))
                        .filter(|ch| ch.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap()
                    // println!("{}", test);
                    // println!("{}", string);
                    // test.parse::<u32>().unwrap()
                }),
            )
        };
        let hashed = hash_algorithm(&label);
        match command {
            CommandType::Remove => boxes[hashed as usize].shift_remove(&label),
            CommandType::Assign(num) => boxes[hashed as usize].insert(label, num),
        };
        // dbg!(&boxes[..5]);
    }
    let mut answer: u32 = 0;
    for (i, container) in boxes.iter().enumerate() {
        //cant call it box, have to call it container
        for (y, (_, value)) in container.iter().enumerate() {
            answer += ((i + 1) * (y + 1) * *value as usize) as u32
        }
    }

    answer
}

fn hash_algorithm(input: &str) -> u32 {
    let mut result = 0;
    for ch in input.bytes() {
        //newline
        if ch == 10 {
            continue;
        }
        result += ch as u32;
        result *= 17;
        result %= 256;
    }
    // println!("{}", result);
    result
}
#[derive(Debug)]
enum CommandType {
    Remove,
    Assign(u32),
}
