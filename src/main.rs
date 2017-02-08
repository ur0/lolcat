use std::io;
use std::io::prelude::*;

extern crate clap;
extern crate rand;
use clap::{Arg, App};

mod cat;

fn main() {
    let mut c = parse_cli_args();
    let stdin = io::stdin(); // For lifetime reasons

    for line in stdin.lock().lines() {
        cat::print_with_lolcat(line.unwrap(), &mut c);
    }
}

fn parse_cli_args() -> cat::Control {
    let matches = App::new("lolcat")
        .version("0.1.0")
        .author("Umang Raghuvanshi <u@umangis.me>")
        .about("The good ol' lolcat, now with fearless concurrency.")
        .arg(Arg::with_name("seed")
            .short("s")
            .long("seed")
            .help("A seed for your lolcat. Setting this to 0 randomizes the seed.")
            .takes_value(true))
        .arg(Arg::with_name("spread")
            .short("S")
            .long("spread")
            .help("How much should we spread dem colors? Defaults to 3.0")
            .takes_value(true))
        .arg(Arg::with_name("frequency")
            .short("f")
            .long("frequency")
            .help("Frequency - used in our math. Defaults to 0.1")
            .takes_value(true))
        .get_matches();
    let seed = matches.value_of("seed").unwrap_or("0.0");
    let spread = matches.value_of("spread").unwrap_or("3.0");
    let frequency = matches.value_of("frequency").unwrap_or("0.1");

    let mut seed: f64 = seed.parse().unwrap();
    let spread: f64 = spread.parse().unwrap();
    let frequency: f64 = frequency.parse().unwrap();

    if seed == 0.0 {
        seed = rand::random::<f64>();
    }

    cat::Control {
        seed: seed,
        spread: spread,
        frequency: frequency,
    }
}
