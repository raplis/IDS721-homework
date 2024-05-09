use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("Rust CLI Tool")
        .version("1.0")
        .author("yz803")
        .about("read a file and add each line as a number, then print the total sum")
        .arg(Arg::with_name("input")
             .short('i')
             .long("input")
             .value_name("FILE")
             .help("Sets the input file to use")
             .takes_value(true))
        .get_matches();

    let input = matches.value_of("input").unwrap_or("default.txt");
    println!("Using input file: {}", input);

    if let Err(e) = process_file(input) {
        eprintln!("Error processing file: {}", e);
    }
}

fn process_file(file_path: &str) -> Result<(), io::Error> {
    let input = File::open(file_path)?;
    let buffered = io::BufReader::new(input);

    let mut sum = 0;

    for line in buffered.lines() {
        let line = line?;
        let num: i32 = line.parse().unwrap_or(0);
        sum += num;
    }

    println!("Total sum: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() {
        // test with a file that contains numbers
        let result = process_file("test");
        assert!(result.is_ok());
    }
}