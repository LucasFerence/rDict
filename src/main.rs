use std::process;

use serde_json::Value;

use rdict::file::FileAccess;
use rdict::app::{self, Function, Key, Val};
use rdict::clipboard;
use rdict::Res;

/// Known bugs
/// 1. If there is no file or a file with no brackets, it will not write
/// 
fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main() -> Res<()> {
    let matches = app::app().get_matches();
    let file_access = FileAccess::new();

    // If we have the key and a valid file, continue
    if let (Some(key), Ok(mut v)) = (matches.value_of(Key::name()), file_access.read()) {
        
        // If we have a value/mutable map also, write value to map/file.
        if let (Some(value), Some(map)) = (matches.value_of(Val::name()), v.as_object_mut()) {
            map.insert(
                String::from(key),
                Value::String(String::from(value))
            );  

            // Propogate error if existant
            file_access.write(map)?;

            println!("Wrote value: [{}] for [{}]", value, key);
        } else if let Value::String(val) = &v[key] {
            clipboard::write(val.to_owned())?;
            println!("Value [{}] copied to clipboard", val);
        } else {
            println!("Could not find value key provided!");
        }
    }

    Ok(())
}
