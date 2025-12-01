use std::fs;

pub fn load(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(path) => Ok(path),
        Err(err) => Err(err.to_string()),
    }
}
