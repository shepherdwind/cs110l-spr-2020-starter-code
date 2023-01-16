use std::io::{BufReader, BufRead};
use std::{env, io};
use std::fs::File;
use std::process;

enum WCCommandOption {
    WORDS = 1,
    LINES,
    CHARS,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let options = parse_args(args.get(2));
    let lines = read_file_lines(filename).unwrap();


    let mut data: Vec<usize> = options.iter().map(|opt| {
        match opt {
            WCCommandOption::WORDS => lines.iter().map(|line| line.split(' ').count()).sum(),
            WCCommandOption::LINES => lines.len(),
            WCCommandOption::CHARS => lines.iter().map(|line| line.len()).sum(),
        }
    }).collect();

    data.sort();
    for n in data.iter() {
        print!("\t{}", n);
    }
    println!(" {}", filename);
    return;
}

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

fn parse_args(option: Option<&String>) -> Vec<WCCommandOption> {
    let all = "cwl";
    let options = match option {
        Some(item) => if item.len() > 1 { &item[1..] } else { all },
        None => all,
    };

    let mut result: Vec<WCCommandOption> = Vec::new();
    for opt in options.chars() {
        match opt {
            // m or c
            'm' | 'c' => result.push(WCCommandOption::CHARS),
            'l' => result.push(WCCommandOption::LINES),
            'w' => result.push(WCCommandOption::WORDS),
            _ => (),
        }
    }
    result
}