use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde_json::{Value, json, Map};
use std::fs;

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
        .get_matches();

    let input_file = matches.value_of("input").unwrap();

    let data = fs::read_to_string(input_file).expect("Unable to read file");
    let json_data: Value = serde_json::from_str(&data).expect("Invalid JSON format");

    let mut map_keys: Vec<String> = Vec::new();
    if let Value::Object(ref map) = json_data {
        map_keys = map.keys().cloned().collect();
    }

    if map_keys.is_empty() {
        println!("No keys found in the JSON object. Exiting.");
        return;
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the columns you want to keep")
        .items(&map_keys)
        .interact()
        .unwrap();

    let mut new_json_data = Map::new();
    if let Value::Object(ref map) = json_data {
        for index in selections {
            if let Some(key) = map_keys.get(index) {
                if let Some(value) = map.get(key) {
                    // output for debug
                    println!("Adding key: {}, value: {}", key, value);
                    new_json_data.insert(key.clone(), value.clone());
                }
            }
        }
    }

    // output for debug
    println!("New JSON data: {:?}", new_json_data);

    let output_data = json!(new_json_data);
    let output_data_str = serde_json::to_string(&output_data).expect("Failed to serialize to JSON");
    fs::write("output.json", output_data_str).expect("Unable to write to file");
}
