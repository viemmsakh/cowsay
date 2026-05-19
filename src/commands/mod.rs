use crate::helper;

pub fn cowsay(text: String) {
    let inner_width = helper::calculate_inner_width(&text);
    let dashes = "-".repeat(usize::from(inner_width + 2));
    println!();
    println!(r"/{}\", dashes);
    for line in text.lines() {
        let line = helper::standardize_line(line.to_string(), inner_width);
        let right_padding = inner_width - line.len();
        println!(r"| {}{} |", line, " ".repeat(right_padding));
    }
    println!(r"\{}/", dashes);
    helper::print_horse();
    println!();
}