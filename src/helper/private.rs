use std::process;
use terminal_size::{Width, terminal_size};

pub fn get_max_line_length(text: &String) -> usize {
    let max = match text
            .lines()
            .map(|line| line
                .trim_end()
                .replace('\t', "    ")
                .len()
            ).max() {
        Some(len) => len,
        None => 0,
    };
    max
}

pub fn get_screen_width() -> u16 {
    let size = terminal_size();
    let w = match size {
        Some((w, _h)) => w,
        None => {
            eprintln!("ERROR: tty width not detected.");
            process::exit(1);
        }
    };
    match w {
        Width(value) => value - 2,
    }
}
