use std::fs;

fn solve1(line: String) -> u128 {

    let mut res: u128 = 0;
    for c in line.chars() {
        let check = c.to_ascii_lowercase() as u128 - '0'.to_ascii_lowercase() as u128;
        if check < 10 {
            res += check * 10;
            break;
        }
    }
    
    for c in line.chars().rev() {
        let check = c.to_ascii_lowercase() as u128 - '0'.to_ascii_lowercase() as u128;
        if check < 10 {
            res += check;
            break;
        }
    }

    res
}

fn first_occ(line: String) -> u128 {
    let mut val: u128 = 0;
    let mut min_id: u128 = 10000000;
    let checkvals = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let nums_str = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for id in 0..checkvals.len() {
        let index = line.find(checkvals[id]);
        if let Some(i) = index {
            if (i as u128) < min_id {
                min_id = i as u128;
                val = (id + 1) as u128;
            }
        }

        let index = line.find(nums_str[id]);
        if let Some(i) = index {
            if (i as u128) < min_id {
                min_id = i as u128;
                val = (id + 1) as u128;
            }
        }
    }

    val
}

fn last_occ(line: String) -> u128 {
    let mut val: u128 = 0;
    let mut min_id: u128 = 10000000;
    let checkvals = ["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
    let nums_str = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for id in 0..checkvals.len() {
        let index = line.find(checkvals[id]);
        if let Some(i) = index {
            if (i as u128) < min_id {
                min_id = i as u128;
                val = (id + 1) as u128;
            }
        }

        let index = line.find(nums_str[id]);
        if let Some(i) = index {
            if (i as u128) < min_id {
                min_id = i as u128;
                val = (id + 1) as u128;
            }
        }
    }

    val 
}

fn solve2(line: String) -> u128 {
    let reversed = line.chars().rev().collect();
    first_occ(line) * 10 + last_occ(reversed)
}

fn main() {
    let file_path = "./input.txt";
    let mut ans: u128 = 0;

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    for line in lines {
        ans += solve2(line.to_string());
    }

    println!("ans = {}", ans);
}