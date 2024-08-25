use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors = extract_error(&text);
    fs::write("errors.txt", errors.join("\n"))?;
    Ok(())
}

fn extract_error(logs: &str) -> Vec<String> {
    let mut ret = vec![];
    for log in logs.lines() {
        if log.starts_with("ERROR") {
            ret.push(log.to_string());
        }
    }
    ret
}