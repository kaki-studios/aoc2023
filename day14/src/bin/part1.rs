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
    let result1 = answer(test_input1);
    assert_eq!(result1, 136);
    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> i32 {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let columns: Vec<Vec<char>> = (0..rows[0].len())
        .into_iter()
        .map(|pos| rows.iter().map(|line| line[pos]).collect())
        .collect();
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
    for rock in round_rocks.iter_mut() {
        if rock.1 == 0 {
            continue;
        }
        // dbg!(&columns[rock.0][rock.1 - 1]);
        while !vec!['O', '#'].contains(&columns[rock.0][rock.1 - 1]) {
            rock.1 -= 1;
            // println!("here");
            if rock.1 == 0 {
                break;
            }
        }
    }
    println!("{:?}", round_rocks);
    //TODO: calculate the load, we have now moved the rocks
    0
}
