use clap::{App, Arg, crate_name, crate_authors, crate_version};

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Key::create())
        .arg(Val::create())
        .arg(Show::create())
}

// Provides default functionality for an argument
pub trait Function {
    fn create() -> Arg<'static, 'static>;

    fn name() -> &'static str;
}

pub struct Key;

impl Function for Key {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .help("Key to lookup")
            .index(1)
            .takes_value(true)
    }

    fn name() -> &'static str {
        "key"
    }
}

pub struct Val;

impl Function for Val {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .help("Value to insert for key")
            .index(2)
            .takes_value(true)
    }

    fn name() -> &'static str {
        "insert"
    }
}

pub struct Show;

impl Function for Show {
    
    fn create() -> clap::Arg<'static, 'static> {
        Arg::with_name(Self::name())
            .help("Show all key/value mappings")
            .long(Self::name())
    }

    fn name() -> &'static str {
        "show"
    }
}
