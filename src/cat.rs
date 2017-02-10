use std;

// A struct to contain info we need to print with every character
pub struct Control {
    pub seed: f64,
    pub spread: f64,
    pub frequency: f64,
}

// A wrapper around colored_print
pub fn print_with_lolcat(s: String, c: &mut Control) {
    let original_seed = c.seed;
    for character in s.chars() {
        c.seed += 1.0;
        colored_print(get_color_tuple(c), character);
    }
    print!("\n"); // A newline, because lines() gave us a single line without it
    c.seed = original_seed; // Reset the seed
}

fn colored_print(colors: (u8, u8, u8), c: char) {
    print!("\x1b[38;2;{};{};{}m{}\x1b[0m",
           colors.0,
           colors.1,
           colors.2,
           c);
}

fn get_color_tuple(c: &Control) -> (u8, u8, u8) {
    let i = c.frequency * c.seed / c.spread;
    let red = i.sin() * 127.00 + 128.00;
    let green = (i + (std::f64::consts::PI * 2.00 / 3.00)).sin() * 127.00 + 128.00;
    let blue = (i + (std::f64::consts::PI * 4.00 / 3.00)).sin() * 127.00 + 128.00;

    (red as u8, green as u8, blue as u8)
}
