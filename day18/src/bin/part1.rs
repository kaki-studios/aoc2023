fn main() {
    let test_input1 = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    let result1 = answer(test_input1);
    assert_eq!(result1, 102);
    println!("Test success! Here\'s the answer:");
    let answer = answer(include_str!("../../input.txt"));
    println!("{}", answer);
}
fn answer(input: &str) -> i32 {
    let instructions: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.splitn(3, " ").collect::<Vec<&str>>()[0..=1].to_vec())
        .collect();
    dbg!(instructions);
    //make field (where you will execute the instructions (dig))
    //idk what size to make it
    0
}
