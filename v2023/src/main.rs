pub mod stars;

struct Arguments {
    star: String,
    filename: String,
}

fn main() {
    let args = parse_args();
    let input = match std::fs::read_to_string(args.filename.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: reading '{}' caused problems", args.filename);
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    let answer = match args.star.as_str() {
        "1.1" => stars::day_01::calibration_numbers(input),
        "1.2" => stars::day_01::calibration_letters(input),
        "2.1" => stars::day_02::count_possible_games(input),
        _ => {
            eprintln!("error: star '{}' does not yet shine", args.star);
            std::process::exit(1);
        }
    };
    println!("answer: {}", answer)
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 && show_help(args[0].clone()) {
        print_help();
        std::process::exit(0);
    }
    if args.len() != 2 {
        print_usage();
        std::process::exit(1);
    }
    Arguments {
        star: args[0].clone(),
        filename: args[1].clone(),
    }
}

fn show_help(arg: String) -> bool {
    arg == "help" || arg == "-h" || arg == "--help"
}

fn print_help() {
    println!("aoc - fixing global snow production");
    println!("usage: aoc <star> <file>");
    println!();
    println!("star: the day and problem");
    println!("file: a path to the input");
    println!();
    println!("example: aoc 1.1 inputs/day01");
    println!("https://adventofcode.com/2023");
}

fn print_usage() {
    eprintln!("aoc - fixing global snow production");
    eprintln!("usage: aoc <star> <file>");
    eprintln!("hint: aoc help");
}
