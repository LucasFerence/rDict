use std::error::Error;

pub const APP_NAME: &str = "rdict";

pub type Res<T> = Result<T, Box<dyn Error>>;