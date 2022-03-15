use std::io::{stdin, stdout, Write};
mod hello_triangle;
mod hello_window;

fn main() {
    print!(
        "Enter project number:
(* denotes an incomplete project)
1. Hello Window
2. Hello Triangle*
> "
    );
    stdout().flush().unwrap();
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => match input.strip_suffix("\n").unwrap() {
            "1" => hello_window::run(),
            "2" => hello_triangle::run(),
            _ => eprintln!("Not a valid option"),
        },
        Err(e) => eprintln!("Failed to read line: {}", e),
    }
}
