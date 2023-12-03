use std::{i32, num::ParseIntError, ops::Range};

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
    let line_len = input.lines().collect::<Vec<&str>>().first().unwrap().len();
    println!("{}", line_len);
    let mut iterator = input.chars();
    let mut index = 0;
    while let Some(single) = iterator.next() {
        println!("{}", single);
        if single.is_digit(10) {
            let digits = &input.chars().collect::<Vec<char>>()[index + 1..index + 3];
            println!("first is {}", digits.iter().collect::<String>());
            let mut num = Number::new(digits.iter().collect(), false);
            match num.to_i32() {
                Ok(int) => match int {
                    100.. => {
                        println!("3 digits: {}", int);
                    }
                    _ => {
                        println!("2 digits: {}", int);
                    }
                },
                Err(_) => println!("ParseIntError"),
            }

            // for (y, character) in single.iter().enumerate() {
            //     if let Some(symbol) = input.chars().nth(index + y) {
            //         if symbol == *character {
            //             println!("good");
            //         }
            //     }
            // }
            // if let Some(symbol) = input.chars().nth(index + line_len) {
            //     if !symbol.is_digit(10) {
            //         if symbol != '.' {
            //             sum += num.to_i32();
            //             num.is_valid = true;
            //         }
            //     }
            // }
            iterator.nth(index + 3);
            continue;
        }
        index += 1;
    }
    sum
}

#[derive(Clone)]
struct Number {
    digits: String,
    is_valid: bool,
}

impl Number {
    fn new(digits: String, is_valid: bool) -> Number {
        Number { digits, is_valid }
    }
    fn to_i32(&self) -> Result<i32, ParseIntError> {
        let mut ret: String = String::default();
        for i in self.digits.chars() {
            if i.is_digit(10) {
                ret += &i.to_string();
            }
        }
        println!("{}", ret);
        ret.parse::<i32>()
    }
}

fn is_symbol(character: char) -> bool {
    if character.is_digit(10) || character == '.' {
        return false;
    }
    true
}
