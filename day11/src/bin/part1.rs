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

    let result1 = answer(test_input1);
    assert_eq!(result1, 374);
    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}
#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Clone)]
struct IVec2 {
    x: i32,
    y: i32,
}

impl IVec2 {
    fn new(x: i32, y: i32) -> IVec2 {
        IVec2 { x, y }
    }
}

fn answer(input: &str) -> i32 {
    let mut galaxy_plane: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut empty_rows: Vec<usize> = vec![];
    for (i, line) in galaxy_plane.iter().enumerate() {
        if line.iter().all(|ch| ch == &'.') {
            empty_rows.push(i);
        }
    }
    let empty_row: Vec<char> = ".".repeat(galaxy_plane[0].len()).chars().collect();
    for (index, i) in empty_rows.iter().enumerate() {
        galaxy_plane.insert(*i + index, empty_row.clone());
    }

    let mut empty_columns: Vec<usize> = vec![];
    let mut columns: Vec<Vec<char>> = (0..galaxy_plane[0].len())
        .into_iter()
        .map(|pos| galaxy_plane.iter().map(|line| line[pos]).collect())
        .collect();
    for (i, column) in columns.iter().enumerate() {
        if column.iter().all(|ch| ch == &'.') {
            empty_columns.push(i);
        }
    }
    let empty_column: Vec<char> = ".".repeat(columns[0].len()).chars().collect();
    for (index, i) in empty_columns.iter().enumerate() {
        columns.insert(*i + index, empty_column.clone());
    }
    //throw it back to galaxy_plane
    galaxy_plane = (0..columns[0].len())
        .into_iter()
        .map(|pos| columns.iter().map(|column| column[pos]).collect())
        .collect();
    let galaxies: Vec<IVec2> = galaxy_plane
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == &'#' {
                        Some(IVec2::new(x as i32, y as i32))
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
    let answer: u32 = pairs
        .iter()
        .map(|(p1, p2)| distance(p1.clone(), p2.clone()))
        .sum();
    println!("{:?}", galaxy_plane);
    println!("{:?}", galaxies);
    println!("pairs len: {}", pairs.len());
    println!(
        "{}",
        distance(
            galaxies.last().unwrap().clone(),
            galaxies.first().unwrap().clone()
        )
    );
    dbg!(empty_columns, empty_rows);

    let test_string = galaxy_plane
        .iter()
        .map(|line| {
            let mut string = line.iter().collect::<String>();
            string.push('\n');
            string
        })
        .fold(String::new(), |mut acc, line| {
            acc.extend(line.chars());
            acc
        });
    println!("{}", test_string);

    answer as i32
}

fn distance(a: IVec2, b: IVec2) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}
