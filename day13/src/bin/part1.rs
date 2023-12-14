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
    assert_eq!(result1, 405);
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
        let vertical = reflections(columns).unwrap_or(0);
        let horizontal = reflections(rows).unwrap_or(0);
        dbg!(vertical.clone(), horizontal.clone());
        ans += vertical + horizontal * 100;
    }
    ans as i32
}

fn reflections(lines: Vec<Vec<char>>) -> Option<usize> {
    let mut possible_reflections: Vec<usize> = vec![];
    for (index, pair) in lines.windows(2).enumerate() {
        if pair[0] == pair[1] {
            possible_reflections.push(index)
        }
    }
    let mut ans: Vec<usize> = possible_reflections
        .iter()
        .filter(|index| {
            let max = min(lines.len() - **index - 1, **index + 1);
            let bottom_index = **index;
            let top_index = **index + 1;

            for i in 0..max {
                println!("{}", bottom_index - i);
                dbg!(&lines[top_index + i], &lines[bottom_index - i]);
                if lines[top_index + i] != lines[bottom_index - i] {
                    return false;
                }
            }
            true
        })
        .map(|n| *n)
        .collect();
    if ans.first().is_some() {
        ans[0] += 1;
    }
    ans.first().copied()
}
