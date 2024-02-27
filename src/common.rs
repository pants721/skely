use anyhow::{anyhow, Result};

pub fn is_yes(input: &str) -> Result<bool> {
    let input = input.to_lowercase();
    if input == "yes" || input == "y" {
        Ok(true)
    } else if input == "no" || input == "n" {
        Ok(false)
    } else {
        Err(anyhow!("Invalid user input"))
    }
}

