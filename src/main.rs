use std::env;
use std::io::{ self, BufRead };

fn main() {
    let readme: &str =
        "Usage: non [-r | -w | -h]
    
This program reads input from stdin, concatenates lines by removing newline characters (default), and prints the unwrapped output to stdout.

Options:
    -r, --carriage-return  Remove carriage return characters (\\r) and concatenate lines.
    -w, --windows          Remove Windows-style newline characters (\\r\\n) and concatenate lines.
    -h, --help             Show this help message.

Examples:
    cat input.txt | non
    cat input.txt | non -r
    cat input.txt | non -w
";

    let args: Vec<String> = env::args().collect();
    let handle_cr: bool =
        args.contains(&"-r".to_string()) || args.contains(&"--carriage-return".to_string());
    let handle_windows: bool =
        args.contains(&"-w".to_string()) || args.contains(&"--windows".to_string());
    let show_help: bool = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());

    if show_help {
        println!("{}", readme);
        return;
    }

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                let unwrapped: String = if handle_cr {
                    input.replace("\r", "")
                } else if handle_windows {
                    input.replace("\r\n", "")
                } else {
                    input.replace("\n", "")
                };

                print!("{}", unwrapped);
            }
            Err(err) => {
                eprintln!("Failed to read from stdin: {}", err);
                println!("{}", readme);
            }
        }
    }
}
