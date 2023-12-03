fn main() {
    let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let result = answer(test_input);
    if result == 2286 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn answer(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let mut rounds: Vec<&str> = line.split(": ").collect();
        //remove the "Game: n"
        rounds.remove(0);
        //get a vec of rounds within a game
        rounds = rounds.first().unwrap().split("; ").collect();
        //for each round...
        for round in rounds.iter() {
            //get the individual turns...
            let turns = round.split(", ").collect::<Vec<&str>>().to_vec();
            //for each turn
            for turn in turns {
                //loop through each character
                println!("{}", turn);
                for (i, _ch) in turn.chars().enumerate() {
                    //get the rest of the turn, starting with the current character
                    let rest = String::from_utf8(turn.as_bytes()[i..].to_vec()).unwrap();
                    //if it starts with green... etc
                    if rest.starts_with("green") {
                        let chars: Vec<u8> = turn.as_bytes().to_vec();
                        let mut num = String::from_utf8(chars[0..=1].to_vec()).unwrap();
                        num = num.trim().to_owned();
                        match num.parse::<i32>() {
                            Ok(count) => {
                                // println!("{}", count);
                                if count > min_green {
                                    min_green = count;
                                }
                            }
                            Err(err) => println!("you got an error: {}", err),
                        }
                    }
                    if rest.starts_with("blue") {
                        let chars: Vec<u8> = turn.as_bytes().to_vec();
                        let mut num = String::from_utf8(chars[0..=1].to_vec()).unwrap();
                        num = num.trim().to_owned();
                        match num.parse::<i32>() {
                            Ok(count) => {
                                // println!("{}", count);
                                if count > min_blue {
                                    min_blue = count;
                                }
                            }
                            Err(err) => println!("you got an error: {}", err),
                        }
                    }
                    if rest.starts_with("red") {
                        let chars: Vec<u8> = turn.as_bytes().to_vec();
                        let mut num = String::from_utf8(chars[0..=1].to_vec()).unwrap();
                        num = num.trim().to_owned();
                        match num.parse::<i32>() {
                            Ok(count) => {
                                // println!("{}", count);
                                if count > min_red {
                                    min_red = count;
                                }
                            }
                            Err(err) => println!("you got an error: {}", err),
                        }
                    }
                }
            }
        }
        result += min_green * min_red * min_blue;
    }
    result
}
