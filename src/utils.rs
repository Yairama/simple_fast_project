use std::io::{Read, Result};
use std::process::Child;

pub fn format_project_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => c,
            ' ' | '_' | '.' => '-',
            _ => '-',
        })
        .collect()
}

pub fn read_error_message(output: &mut Child) -> Result<String> {
    let mut error_message = String::new();
    if let Some(mut stderr) = output.stderr.take() {
        stderr.read_to_string(&mut error_message)?;
    }
    Ok(error_message)
}
