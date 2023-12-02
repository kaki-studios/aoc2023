fn main() {
    let input = include_str!("../../input.txt");
    let mut result: u32 = 0;
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input.lines() {
        let mut vec = vec![];

        for (i, char) in line.chars().enumerate() {
            let b = String::from_utf8(line.as_bytes()[i..].to_vec()).unwrap();
            for (e, val) in digits.iter().enumerate() {
                if b.starts_with(val) {
                    vec.push((e + 1) as u32);
                }
            }
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
