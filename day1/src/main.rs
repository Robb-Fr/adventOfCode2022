use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        let mut max: Vec<u32> = vec![0, 0, 0];
        let mut current = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(cal) = line {
                if !cal.trim().is_empty() {
                    current += cal
                        .to_string()
                        .parse::<u32>()
                        .expect("Should be a parsable integer")
                } else {
                    let mut min = max[0];
                    let mut min_index = 0;
                    for (index, &x) in max.iter().enumerate() {
                        if x < min {
                            min = x;
                            min_index = index;
                        }
                    }
                    if current > min {
                        max[min_index] = current;
                        println!("New top 3 : {:?}", max);
                    }
                    current = 0;
                }
            }
        }
        let total_max: u32 = max.iter().sum();
        println!("Total max : {total_max}")
    }
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
