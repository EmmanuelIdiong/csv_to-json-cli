use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs};
use csv::{Reader};

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: String,
    state: String,
    gender: String,
}

fn read_csv (input: &str) -> Vec<Person> {
    let mut rdr = Reader::from_path(input).expect("failed to read csv");
    let mut people = Vec::new();
    for result in rdr.deserialize() {
        let record: Person = result.expect("Invalid row");
        people.push(record);
    }
    return people;
}

fn write_json(output: &str, data: &Vec<Person>) {
    let json = serde_json::to_string_pretty(data).expect("Failed to convert to json");
    fs::write(output, json).expect("Failed to write");
}

fn main() {
        let args = Args::parse();
        let people = read_csv(&args.input);
        write_json(&args.output, &people);
}