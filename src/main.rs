use std::io;

extern crate clap;
extern crate rand;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod cat;

fn main() {
    let mut filename: String = "".to_string();
    let mut c = parse_cli_args(&mut filename);
    let stdin = io::stdin(); // For lifetime reasons

    if filename == "" {
        for line in stdin.lock().lines() {
            cat::print_with_lolcat(line.unwrap(), &mut c);
        }
    } else {
        match lolcat_file(&filename, &mut c) {
            Err(_) => println!("Error opening file {}.", filename),
            _ => {}
        }
    }
}

fn lolcat_file(filename: &String, c: &mut cat::Control) -> Result<(), io::Error> {
    let f = try!(File::open(filename));
    let file = BufReader::new(&f);
    for line in file.lines() {
        cat::print_with_lolcat(line.unwrap(), c);
    }
    Ok(())
}

fn parse_cli_args(filename: &mut String) -> cat::Control {
    let matches = App::new("lolcat")
        .version("1.0.1")
        .author("Umang Raghuvanshi <u@umangis.me>")
        .about("The good ol' lolcat, now with fearless concurrency.")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .help("A seed for your lolcat. Setting this to 0 randomizes the seed.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("spread")
                .short("S")
                .long("spread")
                .help("How much should we spread dem colors? Defaults to 3.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("frequency")
                .short("f")
                .long("frequency")
                .help("Frequency - used in our math. Defaults to 0.1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("background")
                .short("B")
                .long("bg")
                .help("Background mode - If selected the background will be rainbow. Default false")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("dialup")
                .short("D")
                .long("dialup")
                .help("Dialup mode - Simulate dialup connection")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("filename")
                .short("i")
                .long("input file name")
                .help("Lolcat this file. Reads from STDIN if missing")
                .takes_value(true)
                .index(1),
        )
        .get_matches();

    let seed = matches.value_of("seed").unwrap_or("0.0");
    let spread = matches.value_of("spread").unwrap_or("3.0");
    let frequency = matches.value_of("frequency").unwrap_or("0.1");
    let background = matches.is_present("background");
    let dialup = matches.is_present("dialup");

    *filename = matches.value_of("filename").unwrap_or("").to_string();

    let mut seed: f64 = seed.parse().unwrap();
    let spread: f64 = spread.parse().unwrap();
    let frequency: f64 = frequency.parse().unwrap();

    if seed == 0.0 {
        seed = rand::random::<f64>() * 10e9;
    }

    cat::Control {
        seed: seed,
        spread: spread,
        frequency: frequency,
        background_mode: background,
        dialup_mode: dialup,
    }
}
