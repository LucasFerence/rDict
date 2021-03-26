use std::path::{Path, PathBuf};
use std::fs::{File, create_dir};

use serde_json::{Map, Value};
use directories::BaseDirs;

use crate::Res;
use crate::APP_NAME;

const FILE_NAME: &str = "data.json";

// Read from the file, return raw value
pub fn read_value(path: &Path) -> Res<Value> {
    let val;
    let file_path = path.join(FILE_NAME);

    if file_path.exists() {
        println!("Found file... reading");

        let file = File::open(file_path)?;
        val = serde_json::from_reader(file)?;
    } else {
        println!("Creating new file at {:?}", file_path);

        let map = Map::new();
        write_map(&map, path)?;
        val = Value::Object(map);
    }  
    
    Ok(val)
}

// Writes serializeable map to file
pub fn write_map(val: &Map<String, Value>, path: &Path) -> Res<()> {
    let file_path = path.join(FILE_NAME);
    let file = File::create(file_path)?;

    serde_json::to_writer_pretty(file, val)?;

    Ok(())
}

// Ensure the correct directories are in place
pub fn get_data_dir() -> Res<PathBuf> {
    if let Some(base) = BaseDirs::new() {
        let data_dir = base.data_dir().join(APP_NAME);

        if !data_dir.exists() {
            create_dir(&data_dir)?;
        }

        Ok(data_dir)
    } else {
        Err(Box::from("Could not create directory"))
    }
}
