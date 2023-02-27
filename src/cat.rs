extern crate rand;

use rand::Rng;

use std::thread::sleep;
use std::time::Duration;

use std::io::stdout;
use std::io::Write;

// A struct to contain info we need to print with every character
pub struct Control {
    pub seed: f64,
    pub spread: f64,
    pub frequency: f64,
    pub background_mode: bool,
    pub dialup_mode: bool,
    pub print_color: bool,
    pub word_wrap: bool,
    pub terminal_width: u16,
}

pub fn print_lines_lol<I: Iterator<Item = S>, S: AsRef<str>>(lines: I, c: &mut Control) {
    if c.terminal_width == 0b11111111_11111111 {
        generic_print_lines_lol::<false, false, _, _>(lines, c);
    } else if !c.word_wrap {
        generic_print_lines_lol::<true, false, _, _>(lines, c);
    } else {
        generic_print_lines_lol::<true, true, _, _>(lines, c);
    }
}

// This used to have more of a reason to exist, however now all its functionality is in
// print_chars_lol(). It takes in an iterator over lines and prints them all.
// At the end, it resets the foreground color
pub fn generic_print_lines_lol<
    const LINE_WRAP: bool,
    const WORD_WRAP: bool,
    I: Iterator<Item = S>,
    S: AsRef<str>,
>(
    lines: I,
    c: &mut Control,
) {
    let mut word_wrap_buffer = String::new();
    for line in lines {
        generic_print_chars_lol::<LINE_WRAP, WORD_WRAP, false, _>(
            line.as_ref().chars().chain(Some('\n')),
            c,
            &mut word_wrap_buffer,
        );
    }
    if c.print_color {
        print!("\x1b[39m");
    }
}

pub fn print_chars_lol<I: Iterator<Item = char>>(
    mut iter: I,
    c: &mut Control,
    constantly_flush: bool,
) {
    let mut word_wrap_buffer: String = String::new();
    if constantly_flush {
        if c.terminal_width == 0b11111111_11111111 {
            generic_print_chars_lol::<false, false, true, _>(&mut iter, c, &mut word_wrap_buffer);
        } else if !c.word_wrap {
            generic_print_chars_lol::<true, false, true, _>(&mut iter, c, &mut word_wrap_buffer);
        } else {
            generic_print_chars_lol::<true, true, true, _>(&mut iter, c, &mut word_wrap_buffer);
        }
    } else {
        if c.terminal_width == 0b11111111_11111111 {
            generic_print_chars_lol::<false, false, false, _>(&mut iter, c, &mut word_wrap_buffer);
        } else if !c.word_wrap {
            generic_print_chars_lol::<true, false, false, _>(&mut iter, c, &mut word_wrap_buffer);
        } else {
            generic_print_chars_lol::<true, true, false, _>(&mut iter, c, &mut word_wrap_buffer);
        }
    }
}

// Takes in s an iterator over characters
// duplicates escape sequences, otherwise prints printable characters with colored_print
// Print newlines correctly, resetting background
// If constantly_flush is on, it won't wait till a newline to flush stdout
pub fn generic_print_chars_lol<
    const LINE_WRAP: bool,
    const WORD_WRAP: bool,
    const CONSTANTLY_FLUSH: bool,
    I: Iterator<Item = char>,
>(
    mut iter: I,
    c: &mut Control,
    word_wrap_buffer: &mut String,
) {
    let mut seed_at_start_of_line = c.seed;
    let mut ignoring_whitespace = c.background_mode;
    let mut printed_chars_on_line: u16 = 0;

    if !c.print_color {
        for character in iter {
            print!("{}", character);
        }
        return;
    }

    while let Some(character) = iter.next() {
        match character {
            // Consume escape sequences
            '\x1b' => {
                // Escape sequences seem to be one of many different categories: https://en.wikipedia.org/wiki/ANSI_escape_code
                // CSI sequences are \e \[ [bytes in 0x30-0x3F] [bytes in 0x20-0x2F] [final byte in 0x40-0x7E]
                // nF Escape seq are \e [bytes in 0x20-0x2F] [byte in 0x30-0x7E]
                // Fp Escape seq are \e [byte in 0x30-0x3F] [I have no idea, but `sl` creates one where the next byte is the end of the escape sequence, so assume that]
                // Fe Escape seq are \e [byte in 0x40-0x5F] [I have no idea, '' though sl doesn't make one]
                // Fs Escape seq are \e [byte in 0x60-0x7E] [I have no idea, '' though sl doesn't make one]
                // Otherwise the next byte is the whole escape sequence (maybe? I can't exactly tell, but I will go with it)
                // We will consume up to, but not through, the next printable character
                // In addition, we print everything in the escape sequence, even if it is a color (that will be overriden)
                // TODO figure out just how these should affect printed_characters_on_line
                print!("\x1b");
                let mut escape_sequence_character = iter
                    .next()
                    .expect("Escape character with no escape sequence after it");
                print!("{}", escape_sequence_character);
                match escape_sequence_character {
                    '[' => loop {
                        escape_sequence_character =
                            iter.next().expect("CSI escape sequence did not terminate");
                        print!("{}", escape_sequence_character);
                        match escape_sequence_character {
                            '\x30'..='\x3F' => continue,
                            '\x20'..='\x2F' => {
                                loop {
                                    escape_sequence_character =
                                        iter.next().expect("CSI escape sequence did not terminate");
                                    print!("{}", escape_sequence_character);
                                    match escape_sequence_character {
                            '\x20' ..= '\x2F' => continue,
                            '\x40' ..= '\x7E' => break,
                            _ => panic!("CSI escape sequence terminated with an incorrect value"),
                            }
                                }
                                break;
                            }
                            '\x40'..='\x7E' => break,
                            _ => panic!("CSI escape sequence terminated with an incorrect value"),
                        }
                    },
                    '\x20'..='\x2F' => loop {
                        escape_sequence_character =
                            iter.next().expect("nF escape sequence did not terminate");
                        print!("{}", escape_sequence_character);
                        match escape_sequence_character {
                            '\x20'..='\x2F' => continue,
                            '\x30'..='\x7E' => break,
                            _ => panic!("nF escape sequence terminated with an incorrect value"),
                        }
                    },
                    //            '\x30' ..= '\x3F' => panic!("Fp escape sequences are not supported"),
                    //            '\x40' ..= '\x5F' => panic!("Fe escape sequences are not supported"),
                    //            '\x60' ..= '\x7E' => panic!("Fs escape sequences are not supported"),
                    // be lazy and assume in all other cases we consume exactly 1 byte
                    _ => (),
                }
            }
            // Newlines print escape sequences to end background prints, and in dialup mode sleep, and
            // reset the seed of the coloring and the value of ignore_whitespace
            '\n' => {
                if WORD_WRAP {
                    print_word_lol(
                        word_wrap_buffer,
                        c,
                        &mut seed_at_start_of_line,
                        &mut ignoring_whitespace,
                        &mut printed_chars_on_line,
                    );
                }
                handle_newline(
                    c,
                    &mut seed_at_start_of_line,
                    &mut ignoring_whitespace,
                    &mut printed_chars_on_line,
                );
            }
            // Do the same thing as normal characters if word wrap is not on. Or if word wrap is on, handle actual printing
            ' ' => {
                if WORD_WRAP {
                    print_word_lol(
                        word_wrap_buffer,
                        c,
                        &mut seed_at_start_of_line,
                        &mut ignoring_whitespace,
                        &mut printed_chars_on_line,
                    );
                    if printed_chars_on_line == c.terminal_width - 1 {
                        handle_newline(
                            c,
                            &mut seed_at_start_of_line,
                            &mut ignoring_whitespace,
                            &mut printed_chars_on_line,
                        );
                    } else {
                        print_char_lol::<LINE_WRAP, WORD_WRAP>(
                            character,
                            c,
                            &mut seed_at_start_of_line,
                            &mut ignoring_whitespace,
                            &mut printed_chars_on_line,
                        );
                    }
                } else {
                    print_char_lol::<LINE_WRAP, WORD_WRAP>(
                        character,
                        c,
                        &mut seed_at_start_of_line,
                        &mut ignoring_whitespace,
                        &mut printed_chars_on_line,
                    );
                }
            }
            // If not an escape sequence or a newline, print a colorful escape sequence and then the
            // character. Or, if word wrap is on, add to the word buffer
            _ => {
                if WORD_WRAP {
                    // If the buffer is fully the width of the terminal, we must print that now
                    if word_wrap_buffer.len() as u16 + 1 == c.terminal_width {
                        print_word_lol(
                            word_wrap_buffer,
                            c,
                            &mut seed_at_start_of_line,
                            &mut ignoring_whitespace,
                            &mut printed_chars_on_line,
                        );
                        handle_newline(
                            c,
                            &mut seed_at_start_of_line,
                            &mut ignoring_whitespace,
                            &mut printed_chars_on_line,
                        );
                    }
                    word_wrap_buffer.push(character);
                } else {
                    print_char_lol::<LINE_WRAP, WORD_WRAP>(
                        character,
                        c,
                        &mut seed_at_start_of_line,
                        &mut ignoring_whitespace,
                        &mut printed_chars_on_line,
                    );
                }
            }
        }

        // If we should constantly flush, flush after each completed sequence, and also reset
        // colors because otherwise weird things happen
        if CONSTANTLY_FLUSH {
            reset_colors(c);
            stdout().flush().unwrap();
        }
    }
}

fn print_char_lol<const LINE_WRAP: bool, const WORD_WRAP: bool>(
    character: char,
    c: &mut Control,
    seed_at_start_of_line: &mut f64,
    ignoring_whitespace: &mut bool,
    printed_chars_on_line: &mut u16,
) {
    if LINE_WRAP && *printed_chars_on_line == c.terminal_width {
        handle_newline(
            c,
            seed_at_start_of_line,
            ignoring_whitespace,
            printed_chars_on_line,
        );
    }
    // In background mode, don't print colorful whitespace until the first printable character
    if *ignoring_whitespace {
        if character.is_whitespace() {
            print!("{}", character);
        } else {
            *ignoring_whitespace = false;
            colored_print(character, c);
        }
    } else {
        colored_print(character, c);
    }

    c.seed += 1.0;
    if LINE_WRAP {
        *printed_chars_on_line += 1;
    }
}

fn print_word_lol(
    word: &mut String,
    c: &mut Control,
    seed_at_start_of_line: &mut f64,
    ignoring_whitespace: &mut bool,
    printed_chars_on_line: &mut u16,
) {
    if *printed_chars_on_line + word.len() as u16 >= c.terminal_width {
        handle_newline(
            c,
            seed_at_start_of_line,
            ignoring_whitespace,
            printed_chars_on_line,
        );
    }
    // In background mode, don't print colorful whitespace until the first printable character
    if *ignoring_whitespace {
        for character in word.chars() {
            if *ignoring_whitespace && character.is_whitespace() {
                print!("{}", character);
            } else {
                *ignoring_whitespace = false;
                colored_print(character, c);
                c.seed += 1.0;
            }
        }
    } else {
        for character in word.chars() {
            colored_print(character, c);
            c.seed += 1.0;
        }
    }
    *printed_chars_on_line += word.len() as u16;
    word.clear();
}

fn handle_newline(
    c: &mut Control,
    seed_at_start_of_line: &mut f64,
    ignoring_whitespace: &mut bool,
    printed_chars_on_line: &mut u16,
) {
    if c.print_color {
        // Reset the background color only, as we don't have to reset the foreground till
        // the end of the program
        // We reset the background here because otherwise it bleeds all the way to the next line
        if c.background_mode {
            print!("\x1b[49m");
        }
    }
    println!();
    if c.dialup_mode {
        let stall = Duration::from_millis(rand::thread_rng().gen_range(30, 700));
        sleep(stall);
    }

    *seed_at_start_of_line += 1.0;
    c.seed = *seed_at_start_of_line; // Reset the seed, but bump it a bit
    *ignoring_whitespace = c.background_mode;
    *printed_chars_on_line = 0;
}

fn reset_colors(c: &Control) {
    if c.print_color {
        // Reset the background color
        if c.background_mode {
            print!("\x1b[49m");
        }

        // Reset the foreground color
        print!("\x1b[39m");
    }
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

fn get_color_tuple(c: &Control) -> (u8, u8, u8) {
    let i = c.frequency * c.seed / c.spread;
    let red = i.sin() * 127.00 + 128.00;
    let green = (i + (std::f64::consts::PI * 2.00 / 3.00)).sin() * 127.00 + 128.00;
    let blue = (i + (std::f64::consts::PI * 4.00 / 3.00)).sin() * 127.00 + 128.00;

    (red as u8, green as u8, blue as u8)
}
