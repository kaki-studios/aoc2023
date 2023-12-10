fn main() {
    let test_input1 = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

    let result1 = answer(test_input1);
    assert_eq!(result1, 4);
    println!("1st test success!");
    let test_input2 = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let result2 = answer(test_input2);
    assert_eq!(result2, 8);
    println!("2nd test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> i32 {
    let char_plane = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("{:?}", char_plane);
    let mut starting_point = (0, 0);
    for (y, line) in char_plane.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if character == &'S' {
                starting_point = (y, x);
            }
        }
    }

    let mut current_pos = starting_point;
    let mut pipe_indices: Vec<(usize, usize)> = vec![starting_point];
    let mut curr_dir: Direction = {
        let up = char_plane[current_pos.0 - 1][current_pos.1];
        if up == 'F' || up == '|' || up == '7' {
            Direction::Up
        } else {
            let down = char_plane[current_pos.0 + 1][current_pos.1];
            if down == '|' || down == 'L' || down == 'J' {
                Direction::Down
            } else {
                let left = char_plane[current_pos.0][current_pos.1 - 1];
                if left == '-' || left == 'L' || left == 'F' {
                    Direction::Left
                } else {
                    let right = char_plane[current_pos.0][current_pos.1 + 1];
                    if right == 'J' || right == '7' || right == '-' {
                        Direction::Right
                    } else {
                        panic!("no adjacents found")
                    }
                }
            }
        }
    };
    println!("going to: {:?}", curr_dir);
    loop {
        // if current_pos == starting_point && pipe_indices.len() > 1 {
        //     break;
        // }
        match curr_dir {
            Direction::Up => {
                let top = current_pos.0 == 0;
                if !top {
                    let up = char_plane[current_pos.0 - 1][current_pos.1];
                    match up {
                        'F' => curr_dir = Direction::Right,
                        '|' => {
                            curr_dir = Direction::Up;
                        }
                        '7' => {
                            curr_dir = Direction::Left;
                        }
                        _ => {}
                    }
                    if up == 'F' || up == '|' || up == '7' {
                        current_pos.0 -= 1;
                        pipe_indices.push(current_pos);
                        continue;
                    } else if up == 'S' {
                        break;
                    }
                }
            }
            Direction::Down => {
                let bottom = current_pos.0 == char_plane.len() - 1;
                if !bottom {
                    let down = char_plane[current_pos.0 + 1][current_pos.1];
                    match down {
                        'L' => {
                            curr_dir = Direction::Right;
                        }
                        '|' => {
                            curr_dir = Direction::Down;
                        }
                        'J' => {
                            curr_dir = Direction::Left;
                        }
                        _ => {}
                    }
                    if down == 'L' || down == '|' || down == 'J' {
                        current_pos.0 += 1;
                        pipe_indices.push(current_pos);
                        continue;
                    } else if down == 'S' {
                        break;
                    }
                }
            }
            Direction::Left => {
                let left_side = current_pos.1 == 0;
                if !left_side {
                    let left = char_plane[current_pos.0][current_pos.1 - 1];
                    match left {
                        'F' => {
                            curr_dir = Direction::Down;
                        }
                        'L' => {
                            curr_dir = Direction::Up;
                        }
                        '-' => {
                            curr_dir = Direction::Left;
                        }
                        _ => {}
                    }
                    if left == 'F' || left == 'L' || left == '-' {
                        current_pos.1 -= 1;
                        pipe_indices.push(current_pos);
                        continue;
                    } else if left == 'S' {
                        break;
                    }
                }
            }
            Direction::Right => {
                let right_side = current_pos.1 == char_plane[0].len() - 1;
                if !right_side {
                    let right = char_plane[current_pos.0][current_pos.1 + 1];
                    match right {
                        '-' => {
                            curr_dir = Direction::Right;
                        }
                        '7' => {
                            curr_dir = Direction::Down;
                        }
                        'J' => {
                            curr_dir = Direction::Up;
                        }
                        _ => {}
                    }
                    if right == '-' || right == 'J' || right == '7' {
                        current_pos.1 += 1;
                        pipe_indices.push(current_pos);
                        continue;
                    } else if right == 'S' {
                        break;
                    }
                }
            }
        }
    }
    println!("{:?}", char_plane[starting_point.0][starting_point.1]);

    println!("gathered the pipe_indices!");

    println!("{:?}", pipe_indices);
    let area = char_plane
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut inside = false;
            line.iter()
                .enumerate()
                .filter(|(x, ch)| {
                    if pipe_indices.contains(&(y, *x)) {
                        if ['S', '|', 'F', '7'].contains(ch) {
                            inside = !inside;
                        };
                        false
                    } else {
                        inside
                    }
                })
                .count()
        })
        .sum::<usize>();
    area as i32
}
#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
