pub fn format_project_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => c,
            ' ' | '-' | '.' => '_',
            _ => '_',
        })
        .collect()
}
