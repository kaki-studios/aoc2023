fn main() {
    let test_input1 = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    let result1 = answer(test_input1);
    assert_eq!(result1, 4);
    println!("1st test success!");
    let test_input2 = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let result2 = answer(test_input2);
    assert_eq!(result2, 8);
    println!("2nd test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> i32 {
    let mut char_plane = input
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
    //start at 1 because starting_point should be counted too
    let mut pipe_count = 1;
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
    loop {
        if current_pos == starting_point && pipe_count != 1 {
            break;
        }
        println!(
            "{}, {:?}, {:?}",
            char_plane[current_pos.0][current_pos.1], curr_dir, current_pos
        );
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
                        pipe_count += 1;
                        println!("{:?}", char_plane);

                        continue;
                    } else if up == 'S' {
                        return pipe_count / 2;
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
                        pipe_count += 1;

                        println!("{:?}", char_plane);
                        continue;
                    } else if down == 'S' {
                        return pipe_count / 2;
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
                        pipe_count += 1;
                        println!("{:?}", char_plane);
                        continue;
                    } else if left == 'S' {
                        return pipe_count / 2;
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
                        pipe_count += 1;
                        println!("{:?}", char_plane);
                        continue;
                    } else if right == 'S' {
                        return pipe_count / 2;
                    }
                }
            }
        }
    }
    println!("{:?}", char_plane[starting_point.0][starting_point.1]);

    pipe_count / 2
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
