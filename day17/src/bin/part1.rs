//NOTE: it goes on forever, i need to nudge it in the direction of the target somehow
//also this "solution" is too greedy, it doesnt look into the future

use std::{char, usize};

fn main() {
    let test_input1 = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let result1 = answer(test_input1);
    assert_eq!(result1, 102);
    println!("Test success! Here\'s the answer:");
    let answer = answer(include_str!("../../input.txt"));
    println!("{}", answer);
}
fn answer(input: &str) -> i32 {
    //go from top left to bottom right without going more than 3 steps in a row
    //an while minimizing the sum of all visited numbers
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("{:?}", rows);
    let mut curr_pos: (usize, usize) = (0, 0);
    let mut possible_dirs: Vec<Direction>;
    let mut answer: i32 = -rows[curr_pos.0][curr_pos.1]
        .to_string()
        .parse::<i32>()
        .unwrap();
    let target = (rows.len() - 1, rows[0].len() - 1);
    let mut prev_dir: Option<Direction> = None;
    let mut curr_dir: Direction;
    let mut continuous_dir: u32 = 0;
    let mut visited_cells: Vec<(usize, usize)> = vec![];
    while curr_pos != target {
        answer += rows[curr_pos.0][curr_pos.1]
            .to_string()
            .parse::<i32>()
            .unwrap();
        let up = curr_pos.0 == 0;
        let down = curr_pos.0 == target.0;
        let left = curr_pos.1 == 0;
        let right = curr_pos.1 == target.1;
        if down && right {
            break;
        }
        let mut possible_steps: Vec<((usize, usize), Direction)> = vec![];
        possible_dirs = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        possible_dirs.retain(|dir| {
            if let Some(prev) = prev_dir {
                dir != match prev {
                    Direction::Right => &Direction::Left,
                    Direction::Left => &Direction::Right,
                    Direction::Down => &Direction::Up,
                    Direction::Up => &Direction::Down,
                }
            } else {
                true
            }
        });
        if continuous_dir == 3 {
            continuous_dir = 0;
            possible_dirs.retain(|dir| {
                if let Some(prev) = prev_dir {
                    dir != &prev
                } else {
                    true
                }
            });
            println!("{:?}", possible_dirs);
        }
        for dir in &possible_dirs {
            match dir {
                Direction::Up => {
                    if !up {
                        possible_steps.push(((curr_pos.0 - 1, curr_pos.1), Direction::Up));
                    }
                }
                Direction::Down => {
                    if !down {
                        possible_steps.push(((curr_pos.0 + 1, curr_pos.1), Direction::Down));
                    }
                }
                Direction::Left => {
                    if !left {
                        possible_steps.push(((curr_pos.0, curr_pos.1 - 1), Direction::Left));
                    }
                }
                Direction::Right => {
                    if !right {
                        possible_steps.push(((curr_pos.0, curr_pos.1 + 1), Direction::Right))
                    }
                }
            }
        }
        possible_steps
            .sort_by_key(|(coord, _)| rows[coord.0][coord.1].to_string().parse::<u32>().unwrap());
        // dbg!(&visited_cells);
        let mut test = possible_steps.clone();

        test.retain(|step| !visited_cells.contains(&step.0));
        if test.len() > 0 {
            possible_steps.retain(|step| !visited_cells.contains(&step.0));
        }
        // dbg!(&possible_steps);
        for (y, line) in rows.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                if curr_pos == (y, x) {
                    print!("0");
                } else if visited_cells.contains(&(y, x)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!("");
        }
        visited_cells.push(curr_pos);
        curr_dir = possible_steps.first().unwrap().1;
        if prev_dir == None {
            prev_dir = Some(curr_dir);
        }
        curr_pos = possible_steps.first().unwrap().0;

        if prev_dir == Some(curr_dir) {
            continuous_dir += 1;
        } else {
            continuous_dir = 0;
        }
        dbg!(&answer);
        println!("---------------");
        prev_dir = Some(curr_dir);
    }
    answer
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
