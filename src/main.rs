use serde_json::Value;
use clap::App;

use rdict::file;
use rdict::arg::{Function, Key, Val};
use rdict::clipboard;
use rdict::Res;
use rdict::APP_NAME;

/// Known bugs
/// 1. If there is no file or a file with no brackets, it will not write
/// 
fn main() -> Res<()> {
    // Construct the app with valid arguments
    let app = App::new(APP_NAME)
        .version("0.1")
        .author("Lucas Ference <ference.lucas@gmail.com>")
        .arg(Key::create())
        .arg(Val::create());

    let matches = app.get_matches();

    let dir = file::get_data_dir()?;

    // If we have the key and a valid file, continue
    if let (Some(key), Ok(mut v)) = (matches.value_of(Key::name()), file::read_value(dir.as_path())) {
        
        // If we have a value/mutable map also, write value to map/file.
        if let (Some(value), Some(map)) = (matches.value_of(Val::name()), v.as_object_mut()) {
            map.insert(
                String::from(key),
                Value::String(String::from(value))
            );

            // Propogate error if existant
            file::write_map(map, dir.as_path())?;
        } else if let Value::String(val) = &v[key] {
            println!("Value [{}] copied to clipboard", val);
            clipboard::write(val.to_owned())?;
        } else {
            println!("Could not find value key provided!");
        }
    }

    Ok(())
}
