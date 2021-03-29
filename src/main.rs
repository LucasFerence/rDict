use std::process;

use serde_json::Value;

use rdict::file::FileAccess;
use rdict::app::{self, Function, Key, Val, Remove, Show};
use rdict::clipboard;
use rdict::Res;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main() -> Res<()> {
    let matches = app::app().get_matches();
    let file_access = FileAccess::new();

    // Read the file before doing anything
    let mut file = file_access.read()?;

    // Check for show flag
    if matches.occurrences_of(Show::name()) > 0 {
        let file_map = file.as_object().ok_or("Could not process immutable file")?;
        for mapping in file_map {
            println!("{:?}", mapping);
        }
    }

    // Check for a key
    if let Some(key) = matches.value_of(Key::name()) {
        // Need the mutable map
        let file_map = file.as_object_mut().ok_or("Could not process mutable file")?;
        
        // Check to see if we wanna remove what is at the key
        if matches.occurrences_of(Remove::name()) > 0 {
            // Remove the key/value pair
            file_map.remove(key);

            file_access.write(file_map)?;
            println!("Removed mapping for [{}]", key)
        } else if let Some(value) = matches.value_of(Val::name()) {
            // Insert the value for the key
            file_map.insert(
                String::from(key),
                Value::String(String::from(value))
            );

            file_access.write(file_map)?;
            println!("Wrote value: [{}] for [{}]", value, key);
        } else if let Value::String(val) = &file[key] {
            // Copy the key to the clipboard if not inserting
            clipboard::write(val.to_owned())?;
            println!("Value [{}] copied to clipboard", val);
        } else {
            println!("Could not find value key provided!");
        }
    }

    Ok(())
}
