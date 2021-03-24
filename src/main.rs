use serde_json::{Result, Value};
use clap::App;

use rdict::file;
use rdict::arg::{Function, Key, Insert};

/// Known bugs
/// 1. If there is no file or a file with no brackets, it will not write
/// 
fn main() -> Result<()> {
    // Construct the app with valid arguments
    let app = App::new("rdict")
        .version("0.1")
        .author("Lucas Ference <ference.lucas@gmail.com>")
        .arg(Key::create())
        .arg(Insert::create());

    let matches = app.get_matches();

    // If we have the key and a valid file, continue
    if let (Some(key), Ok(mut v)) = (matches.value_of(Key::name()), file::read_value()) {
        
        // If we have a value/mutable map also, write value to map/file.
        if let (Some(value), Some(map)) = (matches.value_of(Insert::name()), v.as_object_mut()) {
            map.insert(
                String::from(key),
                Value::String(String::from(value))
            );

            // Propogate error if existant
            file::write_map(map)?;
        } else {
            // Otherwise, just print what's at the location
            println!("{:?}", v[key]);
        }
    }

    Ok(())
}
