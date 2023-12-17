//TODO: make it work (and optimize it)

use std::usize;

fn main() {
    let test_input1 = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let result1 = answer(test_input1, 1);
    assert!(result1 == 87);
    println!("Test success! Here\'s the answer:");
    let answer = answer(include_str!("../../input.txt"), 1_000_000_000);
    println!("{}", answer);
}

fn answer(input: &str, cycles: i32) -> u32 {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let columns: Vec<Vec<char>> = (0..rows[0].len())
        .into_iter()
        .map(|pos| rows.iter().map(|line| line[pos]).collect())
        .collect();
    dbg!(&columns);
    let mut round_rocks: Vec<(usize, usize)> = columns
        .iter()
        .enumerate()
        .map(|(x, column)| {
            column
                .iter()
                .enumerate()
                .filter_map(|(y, ch)| if ch == &'O' { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();
    //we sort it so the northernmost rocks become first in the vec
    round_rocks.sort_by_key(|rock| (rock.1, rock.0));
    println!("{:?}", round_rocks);
    let mut shifted_rocks = vec![];
    for rock in round_rocks.iter() {
        let mut new_rock = rock.clone();
        if new_rock.1 == 0 {
            println!("INTIRNTRITNRSINTSENT");
            println!("INTIRNTRITNRSINTSENT");
            shifted_rocks.push(new_rock);
            continue;
        }
        // dbg!(&columns[rock.0][rock.1 - 1]);
        while columns[new_rock.0][new_rock.1 - 1] != '#' {
            if shifted_rocks.contains(&(new_rock.0, new_rock.1 - 1)) {
                break;
            }
            new_rock.1 -= 1;
            // println!("here");
            if new_rock.1 == 0 {
                break;
            }
            dbg!(&columns[new_rock.0][new_rock.1 - 1]);
            // println!("{}", shifted_rocks.contains(&(new_rock.0, new_rock.1 - 1)));
            println!("{:?}", shifted_rocks);
            for y in 0..rows.len() {
                for x in 0..rows[0].len() {
                    if shifted_rocks.contains(&(x, y)) {
                        print!("0");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
        }
        shifted_rocks.push(new_rock);
    }
    // shifted_rocks.dedup();
    //we sort again it so the northernmost rocks become first in the vec
    shifted_rocks.sort_by_key(|rock| (rock.1, rock.0));
    println!("{:?}", shifted_rocks);
    for y in 0..rows.len() {
        for x in 0..rows[0].len() {
            if shifted_rocks.contains(&(x, y)) {
                print!("0");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    let mut answer = 0;
    let len = columns[0].len();
    for rock in shifted_rocks {
        let load = len - rock.1;
        dbg!(&load);
        answer += load as i32;
    }

    calculate_load(shifted_rocks, columns[0].len())
}

fn calculate_load(input: Vec<(usize, usize)>, height: usize) -> u32 {
    0
}
