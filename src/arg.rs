use clap::Arg;

// Provides default functionality for an argument
pub trait Function {
    fn create() -> Arg<'static, 'static>;

    fn name() -> &'static str;
}

pub struct Key;

impl Function for Key {
    fn create() -> Arg<'static, 'static> {
        Arg::with_name(Key::name())
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
        Arg::with_name(Val::name())
            .help("Value to insert for key")
            .index(2)
            .takes_value(true)
    }

    fn name() -> &'static str {
        "insert"
    }
}
