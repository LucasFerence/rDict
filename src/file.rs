use serde_json::Map;
use std::path::Path;
use std::fs::File;

use serde_json::{Result, Value};

const PATH: &str = "/Users/lucas/Development/rdict/data/dict.json";

// Read from the file, return raw value
pub fn read_value() -> Result<Value> {
    let json_file_path = Path::new(PATH);

    // Probably okay to expect here for now
    let file = File::open(json_file_path).expect("File not found");

    serde_json::from_reader(file)
}

// Writes serializeable map to file
pub fn write_map(val: &Map<String, Value>) -> Result<()> {
    let json_file_path = Path::new(PATH);
    // Probably okay to expect here for now
    let file = File::create(json_file_path).expect("File not found");

    serde_json::to_writer_pretty(file, val)
}
