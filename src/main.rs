extern crate atty;
extern crate clap;
extern crate rand;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

mod cat;

fn main() {
    let mut filename: String = "".to_string();
    let mut c = parse_cli_args(&mut filename);
    let stdin = io::stdin(); // For lifetime reasons

    if filename == "" {
/*        for line in stdin.lock().lines() {
            cat::print_line_lol(line.unwrap(), &mut c);
            if c.dialup_mode {
                let stall = Duration::from_millis(rand::thread_rng().gen_range(30, 200));
                sleep(stall);
            }
        }
*/
        cat::print_lines_lol(stdin.lock().lines(), &mut c);
    } else if lolcat_file(&filename, &mut c).is_err() {
        println!("Error opening file {}.", filename)
    }
}

fn lolcat_file(filename: &str, c: &mut cat::Control) -> Result<(), io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
/*    for line in file.lines() {
        cat::print_line_lol(line.unwrap(), c);

        if c.dialup_mode {
            let stall = Duration::from_millis(rand::thread_rng().gen_range(30, 700));
            sleep(stall);
        }
    }
*/
    cat::print_lines_lol(file.lines(), &mut c);
    Ok(())
}

fn parse_cli_args(filename: &mut String) -> cat::Control {
    let matches = lolcat_clap_app()
        .get_matches();

    if matches.is_present("help") {
        print_rainbow_help(false);
        std::process::exit(0);
    }
    if matches.is_present("version") {
        print_rainbow_help(true);
        std::process::exit(0);
    }

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

fn print_rainbow_help(only_version: bool) {
    let app = lolcat_clap_app();

    let mut help = Vec::new();
    if only_version {
        app.write_version(&mut help).unwrap();
    } else {
        app.write_help(&mut help).unwrap();
    }
    let help = String::from_utf8(help).unwrap();

    let mut default_settings = cat::Control {
        seed: rand::random::<f64>() * 10e9,
        spread: 3.0,
        frequency: 0.1,
        background_mode: false,
        dialup_mode: false,
    };

/*    for line in help.lines() {
        cat::print_line_lol(
            line.to_string(),
            &mut default_settings
        );
    }
*/
    cat::print_lines_lol(help.lines(), &mut default_settings);
}

fn lolcat_clap_app() -> App<'static, 'static> {
    App::new("lolcat")
        .version("1.0.2")
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
        .arg(
            Arg::with_name("help")
                .short("h")
                .long("help")
                .help("Prints help information")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("version")
                .short("V")
                .long("version")
                .help("Prints version information")
                .takes_value(false)
        )
}
