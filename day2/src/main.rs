use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let score = lines
            .map(|line| match line {
                Ok(game) => {
                    let vals: Vec<&str> = game.split(" ").collect();
                    rpc_match_part2(vals[0], vals[1])
                }
                _ => panic!("Cannot parse the received line"),
            })
            .reduce(|v1, v2| v1 + v2)
            .expect("Could not correctly reduce given values");
        println!("Score : {score}")
    }
}

fn rpc_match_part2(adversary: &str, result: &str) -> u32 {
    let adv_index: u8 = match adversary {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("Unexpected value for your adversary"),
    };
    let expected_result: u8 = match result {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("Unexpected value for you"),
    };
    let expected_symbol_value = (adv_index + expected_result + 2) % 3;
    return expected_result as u32 * 3 + expected_symbol_value as u32 + 1;
}

fn rpc_match_part1(adversary: &str, you: &str) -> u32 {
    // Rock <- Paper <- Scissors <- Rock <- ...
    // you beat adversary if it is just before you in the cycle
    let adv_index: u8 = match adversary {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("Unexpected value for your adversary"),
    };
    let your_index: u8 = match you {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("Unexpected value for you"),
    };

    let win_value: u8 = if your_index == (adv_index + 1) % 3 {
        // you win
        6
    } else if adv_index == (your_index + 1) % 3 {
        // you loose
        0
    } else {
        // tie
        3
    };
    return win_value as u32 + your_index as u32 + 1;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
