extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use serde_json::{Value};
use std::fs;
use std::collections::HashMap;

fn main() {
    let matches = App::new("My JSON Processor")
        .version("1.0")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .required(true)
                .help("Sets the input JSON file"),
        )
        .arg(
            Arg::with_name("columns")
                .short("c")
                .long("columns")
                .value_name("COLUMNS")
                .required(true)
                .help("Sets the columns to keep"),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let columns: Vec<&str> = matches
        .value_of("columns")
        .unwrap()
        .split(',')
        .collect();

    let data = fs::read_to_string(input_file).expect("Unable to read file");
    let mut json_data: Value = serde_json::from_str(&data).expect("Invalid JSON format");

    if let Value::Object(ref mut map) = json_data {
        let keys: Vec<String> = map.keys().cloned().collect();
        for key in keys {
            if !columns.contains(&key.as_str()) {
                map.remove(&key);
            }
        }
    }

    let output_data = serde_json::to_string(&json_data).expect("Failed to serialize to JSON");
    fs::write("output.json", output_data).expect("Unable to write to file");
}
