# No\n

![non logo](./non_logo.webp)

## Description

Simple line break and carriage return remover

## Installation via Homebrew (MacOS/Linux - x86_64/arm64)

```bash
brew install stenstromen/tap/non
```

## Download and Run Binary

* For **MacOS** and **Linux**: Checkout and download the latest binary from [Releases page](https://github.com/Stenstromen/non/releases/latest/)
* For **Windows**: Build the binary yourself.

## Build and Run Binary

```bash
cargo build --release
./target/release/non
```

## Usage

```bash
Usage: non [-r | -w | -h]

This program reads input from stdin, concatenates lines by removing newline characters (default), and prints the unwrapped output to stdout.

Options:
    -r, --carriage-return  Remove carriage return characters (\r) and concatenate lines.
    -w, --windows          Remove Windows-style newline characters (\r\n) and concatenate lines.
    -h, --help             Show this help message.

Examples:
    cat input.txt | non
    cat input.txt | non -r
    cat input.txt | non -w
```
