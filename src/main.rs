mod utils;

use anyhow::Result;
use clap::Parser;
use color_print::cprintln;
use pluralizer::pluralize;
use std::collections::BTreeMap;
use utils::get_file_content;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    fr_path_file: std::path::PathBuf,
    sc_path_file: std::path::PathBuf,
}

#[derive(Clone)]
enum History {
    Keep { line: String, num: u32 },
    Insert { line: String, num: u32 },
    Remove { line: String, num: u32 },
    Frontier { x: i32, history: Vec<History> },
}

impl History {
    fn get_x(&self) -> i32 {
        match self {
            History::Frontier { x, .. } => *x,
            _ => -1,
        }
    }

    fn get_history(&self) -> Vec<History> {
        match self {
            History::Frontier { history, .. } => history.clone(),
            _ => vec![],
        }
    }

    fn get_line(&self) -> u32 {
        match self {
            History::Keep { num, .. } => *num,
            History::Insert { num, .. } => *num,
            History::Remove { num, .. } => *num,
            _ => 0,
        }
    }
}

fn find_diffs(fr_lines: &Vec<String>, sc_lines: &Vec<String>) -> Vec<History> {
    let mut diffs = Vec::new();

    let mut frontier: BTreeMap<i32, History> = BTreeMap::new();

    frontier.insert(
        1,
        History::Frontier {
            x: 0,
            history: vec![],
        },
    );

    let fr_max = fr_lines.len() as i32;
    let sc_max = sc_lines.len() as i32;

    let mut x: i32;
    let mut y: i32;
    let mut history: Vec<History>;

    for d in 0..(fr_max + sc_max + 1) {
        for k in (-d..d + 1).step_by(2) {
            let go_down =
                if k == -d || (k != d && frontier[&(k - 1)].get_x() < frontier[&(k + 1)].get_x()) {
                    true
                } else {
                    false
                };

            if go_down {
                x = frontier[&(k + 1)].get_x();
                history = frontier[&(k + 1)].get_history();
            } else {
                x = frontier[&(k - 1)].get_x() + 1;
                history = frontier[&(k - 1)].get_history();
            }

            y = x - k;

            if 1 <= y && y <= sc_max && go_down {
                history.push(History::Insert {
                    line: sc_lines[(y - 1) as usize].clone(),
                    num: y as u32,
                });
            } else if 1 <= x && x <= fr_max {
                history.push(History::Remove {
                    line: fr_lines[(x - 1) as usize].clone(),
                    num: x as u32,
                });
            }

            while x < fr_max && y < sc_max && fr_lines[x as usize] == sc_lines[y as usize] {
                x += 1;
                y += 1;
                history.push(History::Keep {
                    line: fr_lines[(x - 1) as usize].clone(),
                    num: x as u32,
                });
            }

            if x >= fr_max && y >= sc_max {
                diffs = history;
                return diffs;
            } else {
                frontier.insert(k, History::Frontier { x, history });
            }
        }
    }

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

    let mut diffs = find_diffs(&fr_lines, &sc_lines);
    let width = ((diffs.len() as f64).log10().floor() + 1.0) as usize;
    let mut total_add: u32 = 0;
    let mut total_del: u32 = 0;

    // Sort the diffs by line number and by type
    diffs.sort_by_key(|history: &History| history.get_line());

    for line in diffs {
        match line {
            History::Keep { line, num } => {
                cprintln!(
                    "{line_num:>width$}: {line_content:}",
                    line_num = num,
                    width = width,
                    line_content = line,
                );
            }
            History::Insert { line, num } => {
                total_add += 1;
                if line.trim().is_empty() {
                    cprintln!(
                        "{line_num:>width$}: <green>+ \\n</>",
                        line_num = num,
                        width = width,
                    );
                } else {
                    cprintln!(
                        "{line_num:>width$}: <green>+ {line_content:}</>",
                        line_num = num,
                        width = width,
                        line_content = line,
                    );
                }
            }
            History::Remove { line, num } => {
                total_del += 1;
                if line.trim().is_empty() {
                    cprintln!(
                        "{line_num:>width$}: <red>- \\n</>",
                        line_num = num,
                        width = width
                    );
                } else {
                    cprintln!(
                        "{line_num:>width$}: <red>- {line_content:}</>",
                        line_num = num,
                        width = width,
                        line_content = line,
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
