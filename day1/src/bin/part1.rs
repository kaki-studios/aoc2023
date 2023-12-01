fn main() {
    let input = include_str!("../../input.txt");
    let mut result: u32 = 0;
    for line in input.split("\n") {
        let mut vec = vec![];
        for char in line.chars() {
            match char.to_digit(10) {
                Some(num) => vec.push(num),
                None => continue,
            }
        }
        if vec.len() >= 1 {
            result += vec[0] * 10 + vec[vec.len() - 1];
        }
    }
    println!("{}", result);
}
