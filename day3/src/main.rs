use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Lines, Result},
    path::Path,
};

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // part_1(lines)
        part_2(lines)
    }
}

fn part_2(lines: Lines<BufReader<File>>) -> () {
    let mut letters_sets: [HashSet<u32>; 3] = Default::default();
    let mut total: u32 = 0;
    for (index, line_res) in lines.enumerate() {
        if let Ok(line) = line_res {
            if index == 0 || index % 3 != 0 {
                letters_sets[index % 3] = collect_line(line);
            } else {
                let filtered = letters_sets[0]
                    .intersection(&letters_sets[1])
                    .filter(|&e| letters_sets[2].contains(e));
                total += filtered.sum::<u32>();
                letters_sets[0] = collect_line(line);
            }
        }
    }
    let filtered = letters_sets[0]
        .intersection(&letters_sets[1])
        .filter(|&e| letters_sets[2].contains(e));
    total += filtered.sum::<u32>();
    println!("Total : {}", total);
}

fn part_1(lines: Lines<BufReader<File>>) -> () {
    let weights = lines
        .map(|line| {
            if let Ok(e) = line {
                let chars_values: Vec<u32> = collect_line(e);
                let n = chars_values.len();
                let left: HashSet<u32> = chars_values[0..n / 2].to_owned().into_iter().collect();
                let right: HashSet<u32> = chars_values[n / 2..n].to_owned().into_iter().collect();
                left.intersection(&right).into_iter().sum::<u32>()
            } else {
                panic!("Failed to parse the line")
            }
        })
        .sum::<u32>();
    println!("Total : {}", weights);
}

fn collect_line<B>(line: String) -> B
where
    B: FromIterator<u32>,
{
    line.chars()
        .map(|c| {
            if c as u32 > 96 {
                // lower-case letter
                c as u32 - 96
            } else {
                // upper case letter
                c as u32 - 38
            }
        })
        .collect()
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
