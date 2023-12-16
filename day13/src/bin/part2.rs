use std::char;
use std::cmp::*;

fn main() {
    let test_input1 = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let result1 = answer(test_input1);
    assert_eq!(result1, 400);
    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> i32 {
    let patterns = input.split("\n\n").collect::<Vec<&str>>();
    let mut ans = 0;
    for pattern in patterns {
        let rows = pattern
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let columns: Vec<Vec<char>> = (0..rows[0].len())
            .into_iter()
            .map(|pos| rows.iter().map(|line| line[pos]).collect())
            .collect();
        //indices start at top right
        let horizontal = reflections(rows).unwrap_or(0);
        let vertical = match horizontal {
            0 => reflections(columns).unwrap_or(0),
            _ => 0,
        };
        dbg!(vertical.clone(), horizontal.clone());
        ans += vertical + horizontal * 100;
    }
    ans as i32
}

fn reflections(lines: Vec<Vec<char>>) -> Option<usize> {
    let mut possible_reflections: Vec<usize> = vec![];
    let mut previous: Vec<char> = vec![];
    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            previous = line.to_vec();
            continue;
        }
        let differences: Vec<usize> = line
            .iter()
            .enumerate()
            .filter(|(i, ch)| **ch != previous[*i])
            .map(|(i, _)| i)
            .collect();
        // if differences.len() > 0 {
        //     line[differences[0]] = previous[differences[0]];
        // }
        // dbg!(&differences);
        // dbg!(&difference);
        if differences.len() <= 1 {
            possible_reflections.push(index - 1);
        }

        previous = line.to_vec();
        // if pair[0] == pair[1] {
        //     possible_reflections.push(index)
        // }
    }
    dbg!(&possible_reflections);
    let ans: Vec<usize> = possible_reflections
        .iter()
        .filter(|index| {
            // println!("======================next possible_reflection: {}", index);
            // let max = min(lines.len() - *index - 1, *index + 1);
            // let bottom_index = *index;
            // let top_index = *index + 1;
            let mut differences = 0;
            for (bottom, top) in lines[0..=**index].iter().rev().zip(&lines[**index + 1..]) {
                let difference: Vec<usize> = top
                    .iter()
                    .enumerate()
                    .filter(|(index, ch)| **ch != bottom[*index])
                    .map(|x| x.0)
                    .collect();
                dbg!(&difference);
                // if difference.len() > 0 {
                //     top[*difference.first().unwrap()] = bottom[*difference.first().unwrap()];
                // }
                differences += difference.len();
            }
            // let mut smudge = false;
            // for i in 1..max {
            //     // println!("------------------------------ next step");
            //     let difference: Vec<usize> = lines[top_index + i]
            //         .iter()
            //         .enumerate()
            //         .filter(|(index, ch)| **ch != lines[bottom_index - i][*index])
            //         .map(|x| x.0)
            //         .collect();
            //     dbg!(&difference);
            //     if difference.len() > 0 {
            //         lines[top_index + i][*difference.first().unwrap()] =
            //             lines[bottom_index - i][*difference.first().unwrap()];
            //     }
            //     differences += difference.len();
            //     // if i == 0 {
            //     //     difference = 0;
            //     // }
            //     // println!("current: {}", difference);
            //     // let filter: i32 = 1 - difference as i32;
            //     // dbg!(&lines[top_index + i], &lines[bottom_index - i]);
            //     // if filter <= 0 && smudge == true {
            //     //     return false;
            //     // }
            //     //
            //     // if filter == 0 && smudge == false {
            //     // dbg!(&lines[top_index + i], &lines[bottom_index - i]);
            //     //     smudge = true;
            //     // }
            // }
            dbg!(&differences);
            differences == 1
        })
        .map(|i| *i)
        .collect();
    match ans.first() {
        Some(element) => Some(element + 1),
        None => None,
    }
}
