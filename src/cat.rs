extern crate rand;

use std;

// A struct to contain info we need to print with every character
pub struct Control {
    pub seed: f64,
    pub spread: f64,
    pub frequency: f64,
    pub background_mode: bool,
    pub dialup_mode: bool,
}

// A wrapper around write_with_lolcat
pub fn print_with_lolcat(s: String, c: &mut Control) {
    write_with_lolcat(std::io::stdout(), s, c).unwrap();
}

// A wrapper around colored_write
pub fn write_with_lolcat<T: std::io::Write>(mut w: T, s: String, c: &mut Control) -> std::io::Result<()> {
    let original_seed = c.seed;
    let mut skipping = false;
    let mut whitespace_after_newline = true;

    for character in s.chars() {
        // Strip out any color chars
        if character == '\x1b' {
            skipping = true;
            continue;
        }
        if skipping && character == 'm' {
            skipping = false;
            continue;
        }
        if skipping {
            continue;
        }

        if !character.is_whitespace() {
            whitespace_after_newline = false;
        }

        if whitespace_after_newline {
            write!(w, "{}", character)?;
            continue;
        }

        c.seed += 1.0;

        if c.background_mode {
            let bg = get_color_tuple(c);
            let fg = calc_fg_color(bg);
            colored_write_with_background(&mut w, fg, bg, character)?;
        } else {
            let fg = get_color_tuple(c);
            colored_write(&mut w, fg, character)?;
        }
    }

    writeln!(w)?; // A newline, because lines() gave us a single line without it
    c.seed = original_seed + 1.0; // Reset the seed, but bump it a bit

    Ok(())
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
        (12.92 * intensity)
    } else {
        (1.055 * intensity.powf(1.0 / 2.4) - 0.055)
    }
}

fn srgb_to_linear(intensity: f64) -> f64 {
    if intensity < 0.04045 {
        (intensity / 12.92)
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

fn colored_write<T>(mut w: T, fg: (u8, u8, u8), c: char) -> std::io::Result<()>
where
    T: std::io::Write
{
    write!(w, "\x1b[38;2;{};{};{}m{}\x1b[0m", fg.0, fg.1, fg.2, c)
}

fn colored_write_with_background<T>(mut w: T, fg: (u8, u8, u8), bg: (u8, u8, u8), c: char) -> std::io::Result<()>
where
    T: std::io::Write
{
    write!(
        w,
        "\x1b[38;2;{};{};{};48;2;{};{};{}m{}\x1b[0m",
        fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, c
    )
}

// Kept alive but no longer useful
fn colored_print(fg: (u8, u8, u8), c: char) {
    colored_write(std::io::stdout(), fg, c).unwrap();
}

// Kept alive but no longer useful
fn colored_print_with_background(fg: (u8, u8, u8), bg: (u8, u8, u8), c: char) {
    colored_write_with_background(std::io::stdout(), fg, bg, c).unwrap();
}

fn get_color_tuple(c: &Control) -> (u8, u8, u8) {
    let i = c.frequency * c.seed / c.spread;
    let red = i.sin() * 127.00 + 128.00;
    let green = (i + (std::f64::consts::PI * 2.00 / 3.00)).sin() * 127.00 + 128.00;
    let blue = (i + (std::f64::consts::PI * 4.00 / 3.00)).sin() * 127.00 + 128.00;

    (red as u8, green as u8, blue as u8)
}
