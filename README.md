# No\n

## Description

Simple line break and carriage return remover

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
