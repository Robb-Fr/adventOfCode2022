use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Result},
    path::Path,
};

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        println!("{:?}", part_1(lines));
        // part_2(lines)
    }
}

fn part_1(lines: Lines<BufReader<File>>) -> u32 {
    let mut counter = 0;
    for rl in lines {
        if let Ok(l) = rl {
            let vals: Vec<u32> = l
                .split(",")
                .flat_map(|r| r.split("-"))
                .map(|v| v.parse::<u32>().expect("Should be able to parse values"))
                .collect();
            if !(vals[0] > vals[3] || vals[1] < vals[2]) {
                counter += 1;
            }
        }
    }
    counter
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
