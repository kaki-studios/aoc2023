fn main() {
    let test_input1 = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let result1 = answer(test_input1);
    assert_eq!(result1, 114);

    println!("Test success! Here\'s the answer:");
    println!("{}", answer(include_str!("../../input.txt")));
}

fn answer(input: &str) -> i32 {
    let list_of_histories: Vec<Vec<i32>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|history| {
            history
                .split_whitespace()
                .into_iter()
                .map(|num| i32::from_str_radix(num, 10).unwrap())
                .collect()
        })
        .collect();
    // println!("{:?}", list_of_histories);
    let next_values = list_of_histories
        .iter()
        .map(|history| {
            let mut differences: Vec<Vec<i32>> = vec![history.to_vec()];
            //this loop puts the differences in the differences vec
            while !differences
                .last()
                .unwrap()
                .iter()
                .all(|difference| difference == &0)
            {
                differences.push(
                    differences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|a| a[1] - a[0])
                        .collect(),
                );
            }
            let prediction: i32 = differences.iter().rev().fold(0, |mut acc, difference| {
                acc += difference.last().unwrap();
                println!("line: {:?}, prediction: {:?}", difference, acc);
                acc
            });
            prediction
        })
        .collect::<Vec<i32>>();
    next_values.iter().sum::<i32>()
}
