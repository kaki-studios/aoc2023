use std::{i64, mem::swap};

fn main() {
    let test_input1 = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let result1 = answer(test_input1, 100);
    assert_eq!(result1, 8410);
    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt"), 1_000_000));
}
#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Clone)]
struct IVec2 {
    x: i64,
    y: i64,
}

impl IVec2 {
    fn new(x: i64, y: i64) -> IVec2 {
        IVec2 { x, y }
    }
}

fn answer(input: &str, scale: u64) -> i64 {
    let galaxy_plane: Vec<Vec<char>> = input
        .lines()
        .rev()
        .map(|line| line.chars().collect())
        .collect();

    let mut empty_rows: Vec<usize> = vec![];
    for (i, line) in galaxy_plane.iter().enumerate() {
        if line.iter().all(|ch| ch == &'.') {
            empty_rows.push(i);
        }
    }
    let mut empty_columns: Vec<usize> = vec![];
    let columns: Vec<Vec<char>> = (0..galaxy_plane[0].len())
        .into_iter()
        .map(|pos| galaxy_plane.iter().map(|line| line[pos]).collect())
        .collect();
    for (i, column) in columns.iter().enumerate() {
        if column.iter().all(|ch| ch == &'.') {
            empty_columns.push(i);
        }
    }
    let galaxies: Vec<IVec2> = galaxy_plane
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == &'#' {
                        Some(IVec2::new(x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<IVec2>>()
        })
        .flatten()
        .collect();
    let mut pairs: Vec<(IVec2, IVec2)> = vec![];
    for (i, p1) in galaxies.iter().enumerate() {
        for p2 in galaxies[i + 1..].to_vec() {
            println!("{:?}", (p1.clone(), p2.clone()));
            pairs.push((p1.clone(), p2));
        }
    }
    let answer: u64 = pairs
        .iter()
        .map(|(p1, p2)| {
            distance(
                p1.clone(),
                p2.clone(),
                &empty_rows,
                &empty_columns,
                scale as usize,
            )
        })
        .sum();
    // println!("{:?}", galaxy_plane);
    // println!("{:?}", galaxies);
    // println!("pairs len: {}", pairs.len());

    // let test_string = galaxy_plane
    //     .iter()
    //     .rev()
    //     .map(|line| {
    //         let mut string = line.iter().collect::<String>();
    //         string.push('\n');
    //         string
    //     })
    //     .fold(String::new(), |mut acc, line| {
    //         acc.extend(line.chars());
    //         acc
    //     });
    // println!("{}", test_string);
    // println!(
    //     "{}",
    //     distance(
    //         galaxies[0].clone(),
    //         galaxies[1].clone(),
    //         &empty_rows,
    //         &empty_columns,
    //         scale as usize
    //     )
    // );

    answer as i64
}

fn distance(
    mut a: IVec2,
    mut b: IVec2,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
    scale: usize,
) -> u64 {
    if b > a {
        swap(&mut a, &mut b);
    }
    let expanded_b = {
        let expanded_rows = empty_rows.iter().filter(|row| **row < b.y as usize).count();
        let expanded_columns = empty_columns
            .iter()
            .filter(|column| **column < b.x as usize)
            .count();
        let x = b.x + (expanded_columns * (scale - 1)) as i64;
        let y = b.y + (expanded_rows * (scale - 1)) as i64;
        IVec2::new(x, y)
    };
    let expanded_a = {
        let expanded_rows = empty_rows.iter().filter(|row| **row < a.y as usize).count();
        let expanded_columns = empty_columns
            .iter()
            .filter(|column| **column < a.x as usize)
            .count();
        let x = a.x + (expanded_columns * (scale - 1)) as i64;
        let y = a.y + (expanded_rows * (scale - 1)) as i64;
        IVec2::new(x, y)
    };
    ((expanded_a.x - expanded_b.x).abs() + (expanded_a.y - expanded_b.y).abs()) as u64
}
