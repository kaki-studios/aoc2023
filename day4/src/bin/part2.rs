use std::collections::BTreeMap;

fn main() {
    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let result = answer(test_input);

    if result == 30 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}
#[derive(Clone, Copy)]
struct Scratchcard {
    matches: i32,
    copies: i32,
}

fn answer(input: &str) -> i32 {
    let mut sum = 0;
    let mut card_map: Vec<Scratchcard> = vec![];
    for line in input.lines() {
        let split = line.split(": ").collect::<Vec<&str>>();
        // println!("{:?}", split);
        let card = *split.last().unwrap();
        // println!("{card}");
        let nums: Vec<Vec<u32>> = card
            .split(" | ")
            .map(|string| {
                // println!("{string}");
                string
                    .split_whitespace()
                    .map(|num| {
                        // println!("{}", num);
                        num.trim().parse::<u32>().unwrap()
                    })
                    .collect()
            })
            .collect();
        let mut match_count = 0;
        for num_first in nums.first().unwrap() {
            for num_last in nums.last().unwrap() {
                if num_first == num_last {
                    match_count += 1;
                }
            }
        }
        let scratchcard = Scratchcard {
            matches: match_count,
            copies: 1,
        };
        card_map.push(scratchcard);
    }
    let mut copy = card_map.clone();
    for (card_id, scratchcard) in card_map.iter().enumerate() {
        for i in 0..copy[card_id].copies {
            if scratchcard.matches > 0 {
                for i in card_id + 1..=card_id + (scratchcard.matches as usize) {
                    copy[i].copies += 1;
                }
            }
        }
    }
    for (id, card) in copy.iter().enumerate() {
        card_map[id].copies = card.copies;
        sum += card.copies;
    }
    sum
}
