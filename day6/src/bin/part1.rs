fn main() {
    let test_input = "Time:      7  15   30
Distance:  9  40  200";

    let result = answer(test_input);

    if result == 288 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn answer(input: &str) -> u64 {
    let mut result = 1;
    let times: Vec<u64> = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|string| string.trim().parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = input
        .lines()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|string| string.trim().parse::<u64>().unwrap())
        .collect();
    println!("times: {:?}", times);
    println!("distances: {:?}", distances);
    assert_eq!(times.len(), distances.len());
    for i in 0..times.len() {
        let record = distances[i];

        let mut first_way = 0;
        let mut last_way = 0;
        for time_pressed in 0..=times[i] {
            //time pressed == speed
            //distance == speed * time
            let distance = time_pressed * (times[i] - time_pressed);
            if distance > record {
                if first_way == 0 {
                    first_way = time_pressed;
                } else {
                    last_way = time_pressed;
                }
            }
        }

        result *= (first_way..=last_way).count();
    }

    result as u64
}
