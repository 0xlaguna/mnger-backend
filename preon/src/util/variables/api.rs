use once_cell::sync::Lazy;
use std::env;

pub static ENVIRONMENT_NAME: Lazy<String> = Lazy::new(|| {
    env::var("ENVIRONMENT_NAME").expect("Missing ENVIRONMENT_NAME environment variable.")
});
