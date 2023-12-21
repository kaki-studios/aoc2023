//NOTE: doesnt work, even if the test case passes and i have absolutely no idea why
////and im not gonna debug this...
use std::{collections::HashSet, usize};

fn main() {
    let test_input1 = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let result1 = answer(test_input1);
    assert_eq!(result1, 46);
    println!("Test success! Here\'s the answer:");
    let answer = answer(include_str!("../../input.txt"));
    println!("{}", answer);
}
fn answer(input: &str) -> u32 {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut rays: Vec<Ray> = vec![Ray::new((0, 0), Direction::Right)];
    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);
    loop {
        let len = energized_tiles.len();
        let mut new_rays = vec![];
        for ray in rays.iter_mut() {
            let curr = rows[ray.position.0][ray.position.1];
            if curr == '|' && [Direction::Right, Direction::Left].contains(&ray.direction) {
                ray.direction = Direction::Down;
                // new_rays.push(Ray::new(ray.position, Direction::Up));
                let new_ray = Ray::new(ray.position, Direction::Up);
                if !new_rays.contains(&new_ray) {
                    new_rays.push(new_ray);
                }
                // } else if peek == '|' {
            }
            // if peek == '.' {}
            if curr == '/' {
                ray.direction = match ray.direction {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };
            }
            if curr == '\\' {
                ray.direction = match ray.direction {
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                };
            }
            if curr == '-' && [Direction::Up, Direction::Down].contains(&ray.direction) {
                ray.direction = Direction::Right;
                let new_ray = Ray::new(ray.position, Direction::Left);
                if !new_rays.contains(&new_ray) {
                    new_rays.push(new_ray);
                }
                // } else if peek == '-' {
                //     energized_tiles.push(peek_pos);
            }
            let direction_offset: (i32, i32) = match ray.direction {
                Direction::Up => {
                    if ray.position.0 == 0 {
                        continue;
                    }
                    (-1, 0)
                }
                Direction::Left => {
                    if ray.position.1 == 0 {
                        continue;
                    }
                    (0, -1)
                }
                Direction::Down => {
                    if ray.position.0 == rows.len() - 1 {
                        continue;
                    }
                    (1, 0)
                }
                Direction::Right => {
                    if ray.position.1 == rows[0].len() - 1 {
                        continue;
                    }
                    (0, 1)
                }
            };
            let peek_pos = (
                (ray.position.0 as i32 + direction_offset.0) as usize,
                (ray.position.1 as i32 + direction_offset.1) as usize,
            );
            ray.position = peek_pos;
            if !energized_tiles.contains(&peek_pos) {
                energized_tiles.insert(peek_pos);
            }
        }
        rays.extend(new_rays);
        if len == energized_tiles.len() {
            for (y, line) in rows.iter().enumerate() {
                for (x, _ch) in line.iter().enumerate() {
                    if energized_tiles.contains(&(y, x)) {
                        print!("#");
                    } else {
                        print!("{}", _ch);
                    }
                }
                println!("");
            }
            break;
        }
    }
    energized_tiles.len() as u32
}

#[derive(PartialEq, Debug, PartialOrd, Eq, Ord)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}
#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
struct Ray {
    position: (usize, usize),
    direction: Direction,
}

impl Ray {
    fn new(position: (usize, usize), direction: Direction) -> Ray {
        Ray {
            position,
            direction,
        }
    }
}
