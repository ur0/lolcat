extern crate rand;

use atty::Stream;
use std;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

// A struct to contain info we need to print with every character
pub struct Control {
    pub seed: f64,
    pub spread: f64,
    pub frequency: f64,
    pub background_mode: bool,
    pub dialup_mode: bool,
}

pub fn print_lines_lol<I: Iterator<Item=S>, S: AsRef<str>>(lines: I, c: &mut Control) {
    for line in lines {
        print_chars_lol(line.as_ref().chars().chain(Some('\n')), c);
    }
}

// Takes in s an iterator over characters
// duplicates escape sequences, otherwise prints printable characters with colored_print
// Print newlines correctly
// TODO Adds the color to a color escape sequence
fn print_chars_lol<I: Iterator<Item=char>>(mut iter: I, c: &mut Control) {
    let original_seed = c.seed;
    let mut ignore_whitespace = c.background_mode;

    if !atty::is(Stream::Stdout) {
        for s in iter {
            print!("{}", s);
        }
        return;
    }

    while let Some(character) = iter.next() {
        match character {
        // Consume escape sequences
        '\x1b' => {
            // Escape sequences seem to be one of many different categories
            // CSI sequences are \e \[ [bytes in 0x30-0x3F] [bytes in 0x20-0x2F] [final byte in 0x40-0x7E]
            // nF Escape seq are \e [bytes in 0x20-0x2F] [byte in 0x30-0x7E]
            // Fp Escape seq are \e [byte in 0x30-0x3F] [I have no idea, but `sl` creates one where the next byte is the end of the escape sequence, so assume that]
            // Fe Escape seq are \e [byte in 0x40-0x5F] [I have no idea, '' though sl doesn't make one]
            // Fs Escape seq are \e [byte in 0x60-0x7E] [I have no idea, '' though sl doesn't make one]
            // Otherwise the next byte is the whole escape sequence (maybe? I can't exactly tell, but I will go with it)
            // We will consume up to, but not through, the next printable character
            // In addition, we print everything in the escape sequence, even if it is a color (that will be overriden)
            print!("\x1b");
            let mut escape_sequence_character = iter.next().expect("Escape character with no escape sequence after it");
            print!("{}", escape_sequence_character);
            match escape_sequence_character {
            '[' => {
                loop {
                    escape_sequence_character = iter.next().expect("CSI escape sequence did not terminate");
                    print!("{}", escape_sequence_character);
                    match escape_sequence_character {
                    '\x30' ..= '\x3F' => continue,
                    '\x20' ..= '\x2F' => {
                        loop {
                            escape_sequence_character = iter.next().expect("CSI escape sequence did not terminate");
                            print!("{}", escape_sequence_character);
                            match escape_sequence_character {
                            '\x20' ..= '\x2F' => continue,
                            '\x40' ..= '\x7E' => break,
                            _ => panic!("CSI escape sequence terminated with an incorrect value"),
                            }
                        }
                        break;
                    },
                    '\x40' ..= '\x7E' => break,
                    _ => panic!("CSI escape sequence terminated with an incorrect value"),
                    }
                }
            },
            '\x20' ..= '\x2F' => {
                loop {
                    escape_sequence_character = iter.next().expect("nF escape sequence did not terminate");
                    print!("{}", escape_sequence_character);
                    match escape_sequence_character {
                    '\x20' ..= '\x2F' => continue,
                    '\x30' ..= '\x7E' => break,
                    _ => panic!("nF escape sequence terminated with an incorrect value"),
                    }
                }
            },
//            '\x30' ..= '\x3F' => panic!("Fp escape sequences are not supported"),
//            '\x40' ..= '\x5F' => panic!("Fe escape sequences are not supported"),
//            '\x60' ..= '\x7E' => panic!("Fs escape sequences are not supported"),
            // be lazy and assume in all other cases we consume exactly 1 byte
            _ => (),
            }
            continue;
        },
        // Newlines print escape sequences to end background prints, and in dialup mode sleep
        '\n' => {
            if atty::is(Stream::Stdout) {
                // Reset the background color if we should, since otherwise it will bleed all the way to the
                // end of the terminal
                if c.background_mode {
                    print!("\x1b[49m");
                }

                // Reset the foreground color, since we are at the end of the output, and print the last newline
                // This has to happen here, rather than only at the end of the program, because otherwise
                // newlines become weird
                print!("\x1b[39m");
            }
            println!();
            if c.dialup_mode {
                let stall = Duration::from_millis(rand::thread_rng().gen_range(30, 700));
                sleep(stall);
            }
        },
        _ => {
            // In background mode, don't print colorful whitespace until the first printable character
            if ignore_whitespace && character.is_whitespace() {
                print!("{}", character);
                continue;
            } else {
                ignore_whitespace = false;
            }

            colored_print(character, c);
            c.seed += 1.0;
        },
        }
    }

    c.seed = original_seed + 1.0; // Reset the seed, but bump it a bit
}

fn calc_fg_color(bg: (u8, u8, u8)) -> (u8, u8, u8) {
    // Currently, it only computes the forground clolor based on some threshold
    // on grayscale value.
    // TODO: Add a better algorithm for computing forground color
    if conv_grayscale(bg) > 0xA0_u8 {
        (0u8, 0u8, 0u8)
    } else {
        (0xffu8, 0xffu8, 0xffu8)
    }
}

fn linear_to_srgb(intensity: f64) -> f64 {
    if intensity <= 0.003_130_8 {
        12.92 * intensity
    } else {
        1.055 * intensity.powf(1.0 / 2.4) - 0.055
    }
}

fn srgb_to_linear(intensity: f64) -> f64 {
    if intensity < 0.04045 {
        intensity / 12.92
    } else {
        ((intensity + 0.055) / 1.055).powf(2.4)
    }
}

fn conv_grayscale(color: (u8, u8, u8)) -> u8 {
    // See https://en.wikipedia.org/wiki/Grayscale#Converting_color_to_grayscale
    const SCALE: f64 = 256.0;

    // Changing SRGB to Linear for gamma correction
    let red = srgb_to_linear(f64::from(color.0) / SCALE);
    let green = srgb_to_linear(f64::from(color.1) / SCALE);
    let blue = srgb_to_linear(f64::from(color.2) / SCALE);

    // Converting to grayscale
    let gray_linear = red * 0.299 + green * 0.587 + blue * 0.114;

    // Gamma correction
    let gray_srgb = linear_to_srgb(gray_linear);

    (gray_srgb * SCALE) as u8
}

fn colored_print(character: char, c: &mut Control) {
    if c.background_mode {
        let bg = get_color_tuple(c);
        let fg = calc_fg_color(bg);
        print!(
            "\x1b[38;2;{};{};{};48;2;{};{};{}m{}",
            fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, character
        );
    } else {
        let fg = get_color_tuple(c);
        print!("\x1b[38;2;{};{};{}m{}", fg.0, fg.1, fg.2, character);
    }
}

fn get_color_tuple(c: &Control) -> (u8, u8, u8) {
    let i = c.frequency * c.seed / c.spread;
    let red = i.sin() * 127.00 + 128.00;
    let green = (i + (std::f64::consts::PI * 2.00 / 3.00)).sin() * 127.00 + 128.00;
    let blue = (i + (std::f64::consts::PI * 4.00 / 3.00)).sin() * 127.00 + 128.00;

    (red as u8, green as u8, blue as u8)
}
