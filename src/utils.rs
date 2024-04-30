use anyhow::{Context, Result};

pub fn get_file_content(
    file_path: &std::path::PathBuf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = Vec::new();
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("could not read file `{}`", file_path.display()))?;

    for line in content.lines() {
        // Check if line is a empty line
        if line.trim().is_empty() {
            lines.push("".to_string());
        } else {
            lines.push(line.to_string());
        }
    }

    Ok(lines)
}
