extern crate clap;

use clap::Arg;

// Used on any function
pub trait Function {
    fn create() -> Arg<'static, 'static>;

    fn name() -> &'static str;
}

pub struct Key;

impl Function for Key {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Key::name())
            .short("key")
            .help("Key to lookup")
            .takes_value(true)
    }

    fn name() -> &'static str {
        "key"
    }
}
