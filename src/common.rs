#[cfg(test)]
pub mod tests {
    use std::env;
    use std::env::VarError;
    use std::error::Error;

    pub fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();

        let session_id = env::var("AOC_SESSION_ID")?;
        if session_id.is_empty() {
            Err(VarError::NotPresent)?
        }

        let body = client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header("Cookie", format!("session={}", session_id))
            .send()?
            .text()?;

        Ok(body)
    }
}
