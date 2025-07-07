use std::error::Error;

pub mod json;

pub trait OutputFormatter {
    fn format(&self) -> Result<String, Box<dyn Error>>;

    fn name(&self) -> &'static str;

    fn extension(&self) -> &'static str;
}

pub fn get_formatter(format: &str) -> Option<Box<dyn OutputFormatter>> {
    match format.to_lowercase().as_str() {
        "json" => todo!(),
        _ => None,
    }
}

pub fn available_formats() -> Vec<&'static str> {
    vec!["json"]
}
