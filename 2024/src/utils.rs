use std::fs;
use std::path::Path;

pub fn read_input(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("Input file not found: {}", file_path).into());
    }
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}