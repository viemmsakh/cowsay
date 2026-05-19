use std::cmp::min;
mod private;

pub fn calculate_inner_width(text: &String) -> usize {
    let max_string_length = private::get_max_line_length(text);
    let screen_width = private::get_screen_width();
    let max_width = min(usize::from(screen_width) - 2, max_string_length + 2);
    let inner_width = max_width - 2;
    inner_width
}

pub fn print_horse() {
    println!(r"        \   ^__^");
    println!(r"         \  (oo)\_______");
    println!(r"            (__)\       )\/\");
    println!("                ||----w |");
    println!("                ||     ||");
}

pub fn standardize_line(line: String, width: usize) -> String {
    let line = line
                            .trim_end()
                            .replace('\t', "    "); // Tabs break cowsaw. Replace with strings.
    let line = if line.len() > width {
        let mut s = String::from(line);
        s.truncate(width - 6); // -6 to accomodate for " [...]"
        s + " [...]"
    } else {
        String::from(line)
    };
    line
}
