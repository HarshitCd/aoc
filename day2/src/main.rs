use std::{fs, collections::HashMap};


fn solve2(game: String) -> u32 {

    let mut res = 1;
    let mut min_balls: HashMap<String, u32> = HashMap::new(); 
    let processed_game: Vec<&str> = game.split(": ").collect();
    let rounds = processed_game[1].to_string();
    let processed_rounds: Vec<&str> = rounds.split("; ").collect();

    for round in processed_rounds {
        let processed_round: Vec<&str> = round.split(", ").collect();

        for balls in processed_round {
            let balls = balls.to_string();
            let processed_balls: Vec<&str> = balls.split(" ").collect();

            let count: u32 = processed_balls[0].parse().unwrap();
            let val = min_balls.get(&processed_balls[1].to_string());
            match val {
                Some(i) => {
                    if *i < count {
                        min_balls.insert(processed_balls[1].to_string(), count);
                    }
                }
                None => {
                    min_balls.insert(processed_balls[1].to_string(), count); 
                }
            }
            
        }
    }

    for (_, v) in min_balls {
        res *= v;
    }

    return res;
}

fn solve1(bag_content: &HashMap<String, u32>, game: String) -> bool {

    let processed_game: Vec<&str> = game.split(": ").collect();
    let rounds = processed_game[1].to_string();
    let processed_rounds: Vec<&str> = rounds.split("; ").collect();

    for round in processed_rounds {
        let processed_round: Vec<&str> = round.split(", ").collect();

        for balls in processed_round {
            let balls = balls.to_string();
            let processed_balls: Vec<&str> = balls.split(" ").collect();

            let count: u32 = processed_balls[0].parse().unwrap();
            if let Some(i) = bag_content.get(&processed_balls[1].to_string()) {
                if count > *i {
                    return false;
                }
            }
        }
    }

    return true;
}

fn main() {

    let mut bag_content: HashMap<String, u32> = HashMap::new();
    bag_content.insert("red".to_string(), 12);
    bag_content.insert("green".to_string(), 13);
    bag_content.insert("blue".to_string(), 14);

    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let games = content.split("\n");
    let mut res: u32 = 0;
    let mut i: u32 = 0;
    for game in games {
        // i += 1;
        // if solve1(game.to_string()) {
        //     res += i;
        // }

        res += solve2(game.to_string())
    }

    println!("ans = {}", res)
}
