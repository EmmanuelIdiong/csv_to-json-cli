# CSV to JSON CLI (Rust)

A fast command-line tool built in Rust that converts CSV files into structured JSON.

## Features

- Reads CSV files with headers
- Maps rows into typed Rust structs
- Outputs formatted JSON
- Simple CLI usage

## Download (Windows)

Prebuilt Windows executable available in **x86_64-pc-windows-gnu/release**.

Download 'csv_to_json.exe' from the latest release and run:

'''bash
csv_to_json.exe input.csv output.json

## Installation

### Build from Source

'''bash
git clone
[View Source on Github](https://github.com/EmmanuelIdiong/csv_to_json-cli.git)
cd csv_to_json-cli
cargo build --release

## Usage

csv_to_json input.csv output.json

## Example

input.csv:

name,age,city
Emmanuel,19,Lagos

output.json:
[
    {
        "name": "Emmanuel",
        "age": 19,
        "city": "Lagos"
    }
]

Tech Stack:
Rust
Clap
CSV crate
Serde + Serde JSON

Author:
Emmanuel Idiong
