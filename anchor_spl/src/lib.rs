pub mod token_2022;
pub mod token_interface;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
