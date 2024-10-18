extern crate clap;
extern crate rand;
extern crate utf8_chars;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::IsTerminal;
use std::num::ParseIntError;
use utf8_chars::BufReadCharsExt;

mod cat;

const UTF_8_REPLACEMENT: char = '\u{FFFD}';

fn main() {
    let mut filename: String = "".to_string();
    let mut c = parse_cli_args(&mut filename);

    if filename.is_empty() {
        let stdin = io::stdin(); // For lifetime reasons
        cat::print_chars_lol(
            BufReader::new(stdin.lock()).chars().map(|r| r.unwrap_or(UTF_8_REPLACEMENT)),
            &mut c,
            true,
        );
    } else if lolcat_file(&filename, &mut c).is_err() {
        eprintln!("Error opening file {}.", filename)
    }
}

fn lolcat_file(filename: &str, c: &mut cat::Control) -> Result<(), io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
    cat::print_lines_lol(file.lines().map(|r| r.unwrap_or(UTF_8_REPLACEMENT.to_string())), c);
    Ok(())
}

fn parse_cli_args(filename: &mut String) -> cat::Control {
    let matches = lolcat_clap_app().get_matches();

    let seed = matches.value_of("seed").unwrap_or("0.0");
    let spread = matches.value_of("spread").unwrap_or("3.0");
    let frequency = matches.value_of("frequency").unwrap_or("0.1");

    let mut seed: f64 = seed.parse().unwrap();
    let spread: f64 = spread.parse().unwrap();
    let frequency: f64 = frequency.parse().unwrap();

    if seed == 0.0 {
        seed = rand::random::<f64>() * 10e9;
    }

    *filename = matches.value_of("filename").unwrap_or("").to_string();

    let print_color = matches.is_present("force-color") || std::io::stdout().is_terminal();

	// If the terminal width is passed, use that. Else, get the size of the terminal. Else, use 0 (no overflow)
    let terminal_width: Result<u16, ParseIntError> = matches.value_of("width")
        .unwrap_or("")
        .parse();
    let terminal_width: u16 = match terminal_width {
        Ok(width) => width,
        Err(_) => {
            let size = termsize::get();
            match size {
                Some(size) => size.cols,
                None => 0b11111111_11111111,
            }
        }
    };

    let terminal_supports_truecolor = match std::env::var("COLORTERM") {
        Ok(val) => val == "truecolor" || val == "24bit",
        Err(_) => false,
    };

    let mut retval = cat::Control {
        seed,
        spread,
        frequency,
        background_mode: matches.is_present("background"),
        dialup_mode: matches.is_present("dialup"),
        print_color,
        terminal_width_plus_one: terminal_width.wrapping_add(1),
        terminal_supports_truecolor,
    };

    if matches.is_present("help") {
        print_rainbow_help(false, &mut retval);
        std::process::exit(0);
    }
    if matches.is_present("version") {
        print_rainbow_help(true, &mut retval);
        std::process::exit(0);
    }

    retval
}

fn print_rainbow_help(only_version: bool, c: &mut cat::Control) {
    let app = lolcat_clap_app();

    let mut help = Vec::new();
    if only_version {
        app.write_version(&mut help).unwrap();
    } else {
        app.write_help(&mut help).unwrap();
    }
    let help = String::from_utf8(help).unwrap();

    cat::print_lines_lol(help.lines(), c);
}

fn lolcat_clap_app() -> App<'static, 'static> {
    App::new("lolcat")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
            Arg::with_name("force-color")
                .short("F")
                .long("force-color")
                .help("Force color - Print escape sequences even if the output is not a terminal")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("width")
                .long("terminal-width")
                .help("Terminal width - Set a custom terminal wrapping width, or 0 for unlimited")
                .takes_value(true),
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
                .takes_value(false),
        )
        .arg(
            Arg::with_name("version")
                .short("V")
                .long("version")
                .help("Prints version information")
                .takes_value(false),
        )
}
