//TODO: refactor:
//use 2d char array
//first create 2d char array
//loop through it with windows to find all nums
//later, iterate through all nums and validate them
//(much easier with 2d char array)

use std::{
    char,
    collections::{BTreeMap, HashMap},
    i32,
    ops::{Index, Range},
    u32, vec,
};

use itertools::Itertools;

#[derive(Debug)]
enum Character {
    Number(u32),
    Empty,
    Symbol(char),
}

fn main() {
    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let result = answer(test_input);

    if result == 467835 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn answer(input: &str) -> i32 {
    let mut sum = 0;
    let char_map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (y as i32, x as i32),
                    match character {
                        '.' => Character::Empty,
                        c if c.is_ascii_digit() => {
                            Character::Number(c.to_digit(10).expect("shouldnt happen!"))
                        }
                        c => Character::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Character>>();
    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in char_map.iter() {
        if let Character::Number(num) = value {
            match numbers.last() {
                Some(vec) => {
                    let last_num = vec.last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("bad");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("not good!"),
                    }
                }
                None => numbers.push(vec![((*x, *y), *num)]),
            }
            // println!("{x}, {y}");
        }
    }

    for star in char_map
        .iter()
        .filter(|(key, value)| matches!(value, Character::Symbol('*')))
    {
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| {
                //outer_pos.x + pos.x, .y + .x
                (outer_pos.0 + star.0 .1 as i32, outer_pos.1 + star.0 .0)
            })
            .collect();
        let mut indices_of_numbers = vec![];
        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list
                    .iter()
                    .find(|(num_pos, _)| num_pos == &pos)
                    .is_some()
                {
                    indices_of_numbers.push(i)
                }
            }
        }
        let is_gear = indices_of_numbers.iter().unique().count() == 2;
        if is_gear {
            sum += indices_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        };
    }
    //now we have map: the map of the entire input
    //and numbers: a 2d vec of numbers to their indices, basically all numbers in sequence
    sum.try_into().unwrap()
}
