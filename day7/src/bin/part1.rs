use core::panic;
use std::{collections::BTreeMap, i32, thread::park};
use strum::IntoEnumIterator;

fn main() {
    let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let result = answer(test_input);
    //6440
    if result == 6440 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

const CARD_LIST: &str = "AKQJT98765432";

fn reverse_lookup(a: char) -> i32 {
    for (i, character) in CARD_LIST.chars().enumerate() {
        if character == a {
            return i as i32;
        }
    }
    panic!("character not in CARD_LIST");
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, strum::EnumIter)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    string: String,
    bid: i32,
}
impl Hand {
    fn new(hand_str: &str, bid: i32) -> Hand {
        let mut seen_cards: BTreeMap<char, i32> = BTreeMap::new();
        let mut final_type = HandType::FiveOfAKind;
        for card in hand_str.chars() {
            *seen_cards.entry(card).or_insert(0) += 1;
        }
        let mut counts: Vec<i32> = seen_cards.values().cloned().collect::<Vec<i32>>();
        counts.sort_unstable();
        if counts.ends_with(&[5]) {
            final_type = HandType::FiveOfAKind;
        }
        if counts.ends_with(&[4]) {
            final_type = HandType::FourOfAKind;
        }
        if counts.ends_with(&[3, 2]) {
            final_type = HandType::FullHouse;
        } else if counts.ends_with(&[3]) {
            final_type = HandType::ThreeOfAKind;
        }

        if counts.ends_with(&[2, 2]) {
            final_type = HandType::TwoPair;
        } else if counts.ends_with(&[2]) {
            final_type = HandType::OnePair;
        }
        if counts.ends_with(&[1]) {
            final_type = HandType::HighCard;
        }

        return Hand {
            hand_type: final_type,
            string: hand_str.to_owned(),
            bid,
        };
    }
}

fn answer(input: &str) -> i32 {
    let mut result = 0;
    let mut hands: Vec<Vec<Hand>> = vec![];

    for hand_type in HandType::iter().rev() {
        let candidate: Vec<Hand> = input
            .lines()
            .into_iter()
            .map(|line| {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                Hand::new(parts[0], parts[1].parse::<i32>().unwrap())
            })
            .filter(|hand| hand.hand_type == hand_type)
            .collect();
        if !candidate.is_empty() {
            hands.push(candidate);
        }
    }
    for hand_vec in hands.iter_mut() {
        hand_vec.sort_by(|a, b| {
            // println!("{:?}", a.cmp(b));
            a.cmp(b)
        });
    }
    for (i, hand) in hands.iter().flatten().enumerate() {
        println!("Hand: {}", hand.string);
        println!("Hand bid: {}", hand.bid);
        println!("Hand type: {:?}", hand.hand_type);
        println!("Hand rank: {}", i + 1);
        println!("-------------------------");
    }

    hands.iter().flatten().enumerate().for_each(|(i, hand)| {
        println!("hand bid {}", hand.bid);
        println!("rank: {}", i + 1);
        result += (i + 1) as i32 * hand.bid;
    });
    result
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (card_self, card_other) in self.string.chars().zip(other.string.chars()) {
            // println!("{card_self}, other: {card_other}");
            if reverse_lookup(card_self) < reverse_lookup(card_other) {
                return std::cmp::Ordering::Greater;
            } else if reverse_lookup(card_self) > reverse_lookup(card_other) {
                return std::cmp::Ordering::Less;
            } else {
                continue;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
