use clap::App;

use rdict::Function;
use rdict::Key;

fn main() {
    let app = App::new("rDict")
        .version("0.1")
        .author("Lucas Ference <ference.lucas@gmail.com>")
        .arg(Key::create());

    let matches = app.get_matches();

    if let Some(key) = matches.value_of(Key::name()) {
        println!("{:?}", key);
    }
}
