use std::path::Path;
use std::fs::File;

use serde_json::{Result, Value};
use clap::App;
use rdict::arg::{Function, Key};

fn main() -> Result<()> {
    let app = App::new("rDict")
        .version("0.1")
        .author("Lucas Ference <ference.lucas@gmail.com>")
        .arg(Key::create());

    let matches = app.get_matches();

    if let Some(key) = matches.value_of(Key::name()) {
        let json_file_path = Path::new("/Users/lucas/Development/rdict/data/dict.json");
        let file = File::open(json_file_path).expect("uh oh");

        let v: Value = serde_json::from_reader(file)?;
        println!("Read from file: {}", v[key]);
    }

    Ok(())
}
