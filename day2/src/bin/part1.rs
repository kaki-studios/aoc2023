fn main() {
    let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let result = answer(test_input);
    if result == 8 {
        println!("test success! Here\'s the answer:");
        println!("{}", answer(include_str!("../../input.txt")))
    } else {
        println!("failed, try again");
        println!("your result was: {}", result)
    }
}

fn answer(input: &str) -> i32 {
    let mut sum = 0;
    'games: for (game, line) in input.lines().enumerate() {
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
                                if count > 13 {
                                    println!(
                                        "game number {} impossible, because there were {:?} greens",
                                        game + 1,
                                        count
                                    );
                                    continue 'games;
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
                                if count > 14 {
                                    println!(
                                        "game number {} impossible, because there were {:?} blues",
                                        game + 1,
                                        count
                                    );
                                    continue 'games;
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
                                if count > 12 {
                                    println!(
                                        "game number {} impossible, because there were {:?} reds",
                                        game + 1,
                                        count
                                    );
                                    continue 'games;
                                }
                            }
                            Err(err) => println!("you got an error: {}", err),
                        }
                    }
                }
            }
        }
        sum += (game + 1) as i32;
    }
    sum
}
