use grid::Grid; // For lcs()
use std::{env, cmp};
use std::fs::File; // For read_file_lines()
use std::io::{self, BufReader, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut lines : Vec<String>= Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = line?;
        lines.push(line_str);
    }
    Ok(lines)
    // Be sure to delete the #[allow(unused)] line above
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let x = seq1.len();
    let y = seq2.len();
    let mut grid = Grid::new(x + 1, y + 1);

    for i in 0 .. x + 1 {
        for j in 0 .. y + 1 {
            if i == 0 || j == 0 {
                grid.set(i, j, 0);
                continue;
            }

            let str1 = seq1.get(i - 1).unwrap();
            let str2 = seq2.get(j - 1).unwrap();
            let now = if str1 == str2 { 1 } else { 0 };

            let top = grid.get(i - 1, j).unwrap();
            let left = grid.get(i, j - 1).unwrap();
            let cross = grid.get(i - 1, j - 1).unwrap() + now;
            let old = cmp::max(top, left);
            grid.set(i, j, cmp::max(old, cross));
        }
    }
    grid
}

enum DiffResult<'a> {
    Equal(&'a String),
    Left(&'a String),
    Right(&'a String),
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>) {
    let mut i = lines1.len();
    let mut j = lines2.len();

    let mut result: Vec<DiffResult> = Vec::new();

    while i != 0 || j != 0 {
        if i == 0 {
            j = j - 1;
            result.push(DiffResult::Right(&lines2[j]));
            continue;;
        }

        if j == 0 {
            i = i - 1;
            result.push(DiffResult::Left(&lines1[i]));
            continue;;
        }

        let max = lcs_table.get(i, j).unwrap();
        let top = lcs_table.get(i - 1, j).unwrap();
        let left = lcs_table.get(i, j - 1).unwrap();

        if left >= max {
            j = j - 1;
            result.push(DiffResult::Right(&lines2[j]));
            continue;
        }

        if top >= max {
            i = i - 1;
            result.push(DiffResult::Left(&lines1[i]));
            continue;
        }

        if max > top && max > left {
            i = i - 1;
            j = j - 1;
            result.push(DiffResult::Equal(&lines1[i]));
            continue;
        }
    }

    for diff in result.iter().rev() {
        match diff {
            DiffResult::Equal(str) => println!("{}", *str),
            DiffResult::Left(s) => println!("<{}", *s),
            DiffResult::Right(s) => println!(">{}", *s),
        }
    }
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];
    // let filename1 = &"handout-a.txt".to_string();
    // let filename2= &"handout-b.txt".to_string();

    let line1 = read_file_lines(&filename1).unwrap();
    let line2 = read_file_lines(&filename2).unwrap();
    let grid = lcs(&line1, &line2);
    // grid.display();
    print_diff(&grid, &line1, &line2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        print!("result: \n{}\n", &lines.join("\n"));
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
