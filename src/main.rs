use anyhow::{Context, Result};
use clap::Parser;
use color_print::cprintln;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    fr_path_file: std::path::PathBuf,
    sc_path_file: std::path::PathBuf,
}

struct Line {
    line_num: u32,
    line_content: String,
    color: u32,
}

fn get_file_content(
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

fn build_dp_memoized(fr_lines: &Vec<String>, sc_lines: &Vec<String>) -> Vec<Vec<i32>> {
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

fn find_diffs(fr_lines: Vec<String>, sc_lines: Vec<String>) -> Vec<Line> {
    let mut diffs: Vec<Line> = Vec::new();

    let n = fr_lines.len();
    let m = sc_lines.len();

    let dp: Vec<Vec<i32>> = build_dp_memoized(&fr_lines, &sc_lines);

    let mut i = n;
    let mut j = m;

    while i != 0 || j != 0 {
        if i == 0 {
            // Addition
            diffs.push(Line {
                line_num: j as u32,
                line_content: sc_lines.get(j - 1).unwrap().clone(),
                color: 3,
            });
        } else if j == 0 {
            // Deletion
            diffs.push(Line {
                line_num: i as u32,
                line_content: fr_lines.get(i - 1).unwrap().clone(),
                color: 2,
            });
        } else if fr_lines[i - 1] == sc_lines[j - 1] {
            // Common
            diffs.push(Line {
                line_num: i as u32,
                line_content: fr_lines.get(i - 1).unwrap().clone(),
                color: 1,
            });
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] <= dp[i][j - 1] {
            // Addition
            diffs.push(Line {
                line_num: j as u32,
                line_content: sc_lines.get(j - 1).unwrap().clone(),
                color: 3,
            });
            j -= 1;
        } else {
            // Delete
            diffs.push(Line {
                line_num: i as u32,
                line_content: fr_lines.get(i - 1).unwrap().clone(),
                color: 2,
            });
            i -= 1;
        }
    }

    diffs.reverse();

    diffs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(move || {
        println!("You just killed the process!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let args = Cli::parse();

    let fr_lines = get_file_content(&args.fr_path_file)?;
    let sc_lines = get_file_content(&args.sc_path_file)?;

    let diffs = find_diffs(fr_lines, sc_lines);

    for line in diffs {
        match line.color {
            1 => cprintln!("{} {}", line.line_num, line.line_content),
            2 => {
                if line.line_content.trim().is_empty() {
                    cprintln!("{} <red>- \\n</>", line.line_num);
                } else {
                    cprintln!("{} <red>- {}</>", line.line_num, line.line_content);
                }
            }
            3 => {
                if line.line_content.trim().is_empty() {
                    cprintln!("{} <green>+ \\n</>", line.line_num);
                } else {
                    cprintln!("{} <green>+ {}</>", line.line_num, line.line_content);
                }
            }
            _ => (),
        }
    }
    Ok(())
}