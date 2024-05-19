use std::env;
use std::io::{self, Read};

fn main() {
    // Define the readme message
    let readme = "Usage: non [-r | -w | -h]
    
This program reads input from stdin, concatenates lines by removing newline characters (default), and prints the unwrapped output to stdout.

Options:
    -r, --carriage-return  Remove carriage return characters (\\r) and concatenate lines.
    -w, --windows          Remove Windows-style newline characters (\\r\\n) and concatenate lines.
    -h, --help             Show this help message.

Examples:
    cat input.txt | ./non
    cat input.txt | ./non -r
    cat input.txt | ./non -w
";

    // Check for command line arguments
    let args: Vec<String> = env::args().collect();
    let handle_cr = args.contains(&"-r".to_string()) || args.contains(&"--carriage-return".to_string());
    let handle_windows = args.contains(&"-w".to_string()) || args.contains(&"--windows".to_string());
    let show_help = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());

    if show_help {
        println!("{}", readme);
        return;
    }

    // Create a new String to hold the input
    let mut input = String::new();

    // Read the entire input from stdin
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            // Remove the specified characters
            let unwrapped = if handle_cr {
                input.replace("\r", "")
            } else if handle_windows {
                input.replace("\r\n", "")
            } else {
                input.replace("\n", "")
            };

            // Print the unwrapped output without adding a newline
            print!("{}", unwrapped);
        },
        Err(err) => {
            eprintln!("Failed to read from stdin: {}", err);
            println!("{}", readme);
        }
    }
}