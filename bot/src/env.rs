use once_cell::sync::Lazy;

use std::env;

pub static TELEGRAM_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("TELEGRAM_TOKEN").unwrap()
});
