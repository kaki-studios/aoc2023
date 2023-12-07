fn main() {
    let test_input = "Time:      7  15   30
Distance:  9  40  200";

    let result = answer(test_input);

    //46
    if result == 71503 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn answer(input: &str) -> u64 {
    let mut result = 1;
    let time: u64 = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .chars()
        .filter(|character| character.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let record: u64 = input
        .lines()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .chars()
        .filter(|character| character.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut first_way = 0;
    let mut last_way = 0;
    for time_pressed in 0..=time {
        //time pressed == speed
        //distance == speed * time
        let distance = time_pressed * (time - time_pressed);
        if distance > record {
            first_way = time_pressed;
            break;
        }
    }
    for time_pressed in (0..=time).rev() {
        //time pressed == speed
        //distance == speed * time
        let distance = time_pressed * (time - time_pressed);
        if distance > record {
            last_way = time_pressed;
            break;
        }
    }

    result *= (first_way..=last_way).count();

    result as u64
}
