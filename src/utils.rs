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

pub fn build_dp_memoized(fr_lines: &Vec<String>, sc_lines: &Vec<String>) -> Vec<Vec<i32>> {
    let n = fr_lines.len() + 1;
    let m = sc_lines.len() + 1;
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; m]; n];

    for i in 0..n {
        for j in 0..m {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else if fr_lines[i - 1] == sc_lines[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp
}
