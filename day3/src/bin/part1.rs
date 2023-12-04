//TODO: refactor:
//use 2d char array
//first create 2d char array
//loop through it with windows to find all nums
//later, iterate through all nums and validate them
//(much easier with 2d char array)

use std::{
    i32,
    ops::{Index, Range},
    vec,
};

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

    if result == 4361 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn answer(input: &str) -> i32 {
    let mut sum = 0;
    let mut num_list: Vec<Number> = vec![];
    let line_count: usize = input.lines().count();
    let mut skips = 0;
    let line_len: usize = input.lines().collect::<Vec<&str>>().first().unwrap().len();
    let mut char_vec: Vec<Vec<char>> = vec![];
    for (i, val) in input.lines().enumerate() {
        for (y, value) in val.chars().enumerate() {
            char_vec[i].insert(y, value);
        }
    }

    for (i, line) in char_vec.iter().enumerate() {
        for (y, character) in line.windows(3).enumerate() {
            if skips > 0 {
                skips -= 1;
                continue;
            }
            let is_top = i == 0;
            let is_bottom = i == line_count - 1;
            let is_right = y == line_len - 1;
            let is_left = y == 0;
            if character[0].is_digit(10) {
                let num = Number::new(character.iter().collect(), 0, false);
                let number = to_i32(num.digits);
                //TODO: check all digits, not just the first one
                if !is_top {
                    if !char_vec[i - 1].index(y).is_digit(10) && char_vec[i - 1].index(y) != '.' {
                        sum += number;
                        continue;
                    }
                    if !is_right {
                        if !char_vec[i - 1].index(y + 1).is_digit(10)
                            && char_vec[i - 1].index(y + 1) != '.'
                        {
                            sum += number;
                            continue;
                        }
                    }
                    if !is_right {}
                }
                if !is_bottom {
                    if !char_vec[i + 1].index(y).is_digit(10) && char_vec[i + 1].index(y) != '.' {
                        sum += number;
                        continue;
                    }
                }
                if !is_right {
                    if !char_vec[i].index(y + 1).is_digit(10) && char_vec[i].index(y + 1) != '.' {
                        sum += number;
                        continue;
                    }
                }
                if !is_left {
                    if !char_vec[i].index(y - 1).is_digit(10) && char_vec[i].index(y + 1) != '.' {
                        sum += number;
                        continue;
                    }
                }
            }
        }
    }

    'num: for mut num in num_list {
        // sum += num.to_i32();
        // println!("{}", num.digits);
        let vec = &input.chars().collect::<Vec<char>>();
        for (index, _value) in vec[num.range.clone()].iter().enumerate() {
            // ###
            // #0#
            // #X#
            if index + num.range.start + line_len < vec.len() {
                if !vec[index + num.range.start + line_len].is_digit(10)
                    && vec[index + num.range.start + line_len] != '.'
                {
                    num.is_valid = true;
                    sum += to_i32(num.digits);
                    continue 'num;
                }
            }
            // #X#
            // #0#
            // ###
            if index + num.range.start >= line_len {
                if !vec[index + num.range.start - line_len].is_digit(10)
                    && vec[index + num.range.start - line_len] != '.'
                {
                    num.is_valid = true;
                    sum += to_i32(num.digits);
                    continue 'num;
                }
            }
            // ###
            // #0#
            // ##X
            if index + num.range.start + line_len + 1 < vec.len() {
                if !vec[index + num.range.start + line_len + 1].is_digit(10)
                    && vec[index + num.range.start + line_len + 1] != '.'
                {
                    num.is_valid = true;
                    sum += to_i32(num.digits);
                    continue 'num;
                }
            }
            // ###
            // #0#
            // X##
            if index + num.range.start + line_len - 1 < vec.len() {
                if !vec[index + line_len + num.range.start - 1].is_digit(10)
                    && vec[index + num.range.start + line_len - 1] != '.'
                {
                    num.is_valid = true;
                    sum += to_i32(num.digits);
                    continue 'num;
                }
            }
            // X##
            // #0#
            // ###
            if index + num.range.start > line_len {
                if !vec[index + num.range.start - line_len - 1].is_digit(10)
                    && vec[index + num.range.start - line_len - 1] != '.'
                {
                    num.is_valid = true;
                    sum += to_i32(num.digits);
                    continue 'num;
                }
            }
            // ##X
            // #0#
            // ###
            if index + num.range.start >= line_len + 1 {
                if !vec[index + num.range.start - line_len + 1].is_digit(10)
                    && vec[index + num.range.start - line_len + 1] != '.'
                {
                    num.is_valid = true;
                    sum += to_i32(num.digits);
                    println!("{}", vec[index + num.range.start - line_len + 1]);
                    continue 'num;
                }
            }
        }
    }
    sum
}

#[derive(Clone)]
struct Number {
    digits: String,
    range: Range<usize>,
    is_valid: bool,
}

impl Number {
    fn new(raw_digits: String, range_start: usize, is_valid: bool) -> Number {
        let digits: String = raw_digits.chars().filter(|ch| ch.is_digit(10)).collect();
        let range = range_start..range_start + digits.len();
        Number {
            digits,
            range,
            is_valid,
        }
    }
}

fn to_i32(string: String) -> i32 {
    let mut ret: String = String::default();
    for i in string.chars() {
        if i.is_digit(10) {
            ret += &i.to_string();
        }
    }
    ret.parse::<i32>().unwrap()
}
