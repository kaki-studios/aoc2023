use core::panic;
use std::{char, collections::BTreeMap, i32};

fn main() {
    let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let result = answer(test_input);
    //
    if result == 5905 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn score_card(a: char) -> i32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        value => value.to_digit(10).unwrap() as i32,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    string: String,
    bid: i32,
}
impl Hand {
    fn new(hand_str: &str, bid: i32) -> Hand {
        //no jokers
        let mut seen_normal_cards: BTreeMap<char, i32> = BTreeMap::new();
        for card in hand_str.chars().filter(|character| character != &'J') {
            *seen_normal_cards.entry(card).or_insert(0) += 1;
        }
        let jokers: i32 = hand_str
            .chars()
            .filter(|character| character == &'J')
            .count() as i32;
        let final_type;
        let mut counts: Vec<i32> = seen_normal_cards.values().cloned().collect::<Vec<i32>>();
        counts.sort_unstable();
        if jokers != 5 {
            match counts.last_mut() {
                Some(value) => *value += jokers,
                None => println!("weird card: {hand_str}"),
            }
        } else {
            if counts.is_empty() {
                counts.push(5);
            } else {
                println!("weird card: {hand_str}");
            }
        }

        let string = counts
            .iter()
            .map(|count| count.to_string())
            .collect::<String>();
        final_type = match string.as_str() {
            "5" => HandType::FiveOfAKind,
            "14" => HandType::FourOfAKind,
            "23" => HandType::FullHouse,
            "113" => HandType::ThreeOfAKind,
            "122" => HandType::TwoPair,
            "1112" => HandType::OnePair,
            "11111" => HandType::HighCard,
            value => panic!("Invalid card: {value}"),
        };

        return Hand {
            hand_type: final_type,
            string: hand_str.to_owned(),
            bid,
        };
    }

    fn score_hand(&self) -> (i32, i32, i32, i32, i32) {
        let mut result = [0, 0, 0, 0, 0];
        for (i, character) in self.string.chars().enumerate() {
            result[i] = score_card(character);
        }
        result.into()
    }
}

fn answer(input: &str) -> i32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            Hand::new(hand, bid.parse::<i32>().unwrap())
        })
        .collect::<Vec<Hand>>();

    hands.sort_by_key(|hand| (hand.hand_type, hand.score_hand()));
    // for (i, hand) in hands.iter().enumerate() {
    //     println!("Hand: {}", hand.string);
    //     println!("Hand bid: {}", hand.bid);
    //     println!("Hand type: {:?}", hand.hand_type);
    //     println!("Hand rank: {}", i + 1);
    //     println!("-------------------------");
    // }
    let result = hands
        .iter()
        .enumerate()
        .map(|(line, hand)| {
            println!("{}, {:?}", hand.string, hand.hand_type);
            hand.bid * (line as i32 + 1)
        })
        .sum::<i32>();
    result
}
