mod utils;

use anyhow::Result;
use clap::Parser;
use color_print::cprintln;
use pluralizer::pluralize;
use utils::{build_dp_memoized, get_file_content};

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

fn find_diffs(fr_lines: &Vec<String>, sc_lines: &Vec<String>) -> Vec<Line> {
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
            j -= 1;
        } else if j == 0 {
            // Deletion
            diffs.push(Line {
                line_num: i as u32,
                line_content: fr_lines.get(i - 1).unwrap().clone(),
                color: 2,
            });
            i -= 1;
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

    let diffs = find_diffs(&fr_lines, &sc_lines);
    let width = ((diffs.len() as f64).log10().floor() + 1.0) as usize;
    let mut total_add: u32 = 0;
    let mut total_del: u32 = 0;

    for line in diffs {
        match line.color {
            1 => cprintln!(
                "{line_num:>width$}: {line_content:}",
                line_num = line.line_num,
                width = width,
                line_content = line.line_content,
            ),
            2 => {
                total_del += 1;
                if line.line_content.trim().is_empty() {
                    cprintln!(
                        "{line_num:>width$}: <red>- \\n</>",
                        line_num = line.line_num,
                        width = width
                    );
                } else {
                    cprintln!(
                        "{line_num:>width$}: <red>- {line_content:}</>",
                        line_num = line.line_num,
                        width = width,
                        line_content = line.line_content,
                    );
                }
            }
            3 => {
                total_add += 1;
                if line.line_content.trim().is_empty() {
                    cprintln!(
                        "{line_num:>width$}: <green>+ \\n</>",
                        line_num = line.line_num,
                        width = width,
                    );
                } else {
                    cprintln!(
                        "{line_num:>width$}: <green>+ {line_content:}</>",
                        line_num = line.line_num,
                        width = width,
                        line_content = line.line_content,
                    );
                }
            }
            _ => (),
        }
    }

    if total_add + total_del > 0 {
        cprintln!("\n<blue>The files you provided are differents!</>");
        cprintln!("\n<blue>Here is the summary:</>");
        if total_add > 0 {
            cprintln!(
                "<green>+ {} added.</>",
                pluralize("line", total_add.try_into().unwrap(), true)
            );
        }
        if total_del > 0 {
            cprintln!(
                "<red>- {} deleted.</>",
                pluralize("line", total_del.try_into().unwrap(), true)
            );
        }
    } else {
        cprintln!("\n<blue>The files you provided contains the same information!</>");
    }
    Ok(())
}
