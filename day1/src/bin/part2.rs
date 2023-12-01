fn main() {
    let input = include_str!("../../input.txt");
    let mut result: u32 = 0;
    for line in input.lines() {
        let mut vec = vec![];

        for (i, char) in line.chars().enumerate() {
            let b = String::from_utf8(line.as_bytes()[i..].to_vec()).unwrap();
            if b.starts_with("one") {
                vec.push(1);
            }
            if b.starts_with("two") {
                vec.push(2);
            }
            if b.starts_with("three") {
                vec.push(3);
            }
            if b.starts_with("four") {
                vec.push(4);
            }
            if b.starts_with("five") {
                vec.push(5);
            }
            if b.starts_with("six") {
                vec.push(6);
            }
            if b.starts_with("seven") {
                vec.push(7);
            }
            if b.starts_with("eight") {
                vec.push(8);
            }
            if b.starts_with("nine") {
                vec.push(9);
            }

            // b.st
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
