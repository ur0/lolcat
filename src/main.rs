extern crate clap;
extern crate rand;
use clap::{App, Arg};
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

mod cat;

fn main() {
    let mut filename: String = "".to_string();
    let mut c = parse_cli_args(&mut filename);
    let stdin = io::stdin(); // For lifetime reasons

    if filename == "" {
        for line in stdin.lock().lines() {
            cat::print_with_lolcat(line.unwrap(), &mut c);
            if c.dialup_mode {
                let stall = Duration::from_millis(rand::thread_rng().gen_range(30, 200));
                sleep(stall);
            }
        }
    } else if lolcat_file(&filename, &mut c).is_err() {
        println!("Error opening file {}.", filename)
    }
}

fn lolcat_file(filename: &str, c: &mut cat::Control) -> Result<(), io::Error> {
    let f = try!(File::open(filename));
    let file = BufReader::new(&f);
    for line in file.lines() {
        cat::print_with_lolcat(line.unwrap(), c);

        if c.dialup_mode {
            let stall = Duration::from_millis(rand::thread_rng().gen_range(30, 700));
            sleep(stall);
        }
    }
    Ok(())
}

fn parse_cli_args(filename: &mut String) -> cat::Control {
    let app = App::new("lolcat")
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
        );
    let new_help = rainbowize_help(&app);
    let matches = app
        .help(new_help.as_str()) // Set here for lifetimes reasons
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
        seed,
        spread,
        frequency,
        background_mode: background,
        dialup_mode: dialup,
    }
}

fn rainbowize_help<'a, 'b>(app: &App<'a, 'b>) -> String {
    let mut old_help = Vec::new();
    app.write_help(&mut old_help).unwrap();
    let old_help = String::from_utf8(old_help).unwrap();

    let mut new_help = Vec::new();
    let mut c = cat::Control {
        seed: rand::random::<f64>() * 10e9,
        spread: 3.0,
        frequency: 0.1,
        background_mode: false,
        dialup_mode: false,
    };

    for i in old_help.lines() {
        cat::write_with_lolcat(
            &mut new_help,
            String::from(i), // extra-copy made here :/
            &mut c
        )
        .unwrap();
    }

    String::from_utf8(new_help).unwrap()
}
