use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = if args.len() > 1 {
        String::from("Hello, ".to_owned() + &args[1] + "!")
    } else {
        String::from("Hello, World!")
    };

    let width = message.chars().count();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

