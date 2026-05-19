use clap::Parser;

/// Simple CLI to display text with a cow.
#[derive(Parser, Debug)]
#[command(version, about = "A fun CLI to display text with a cow.", long_about = None)]
pub struct Args {
    /// String you want the cow to say. Also accepts STDIN.
    #[arg(value_name = "STRING")]
    pub text: Option<String>,
}