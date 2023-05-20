use clap::{App, Arg};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

struct Cli {
    purge_all: bool,
    path: Option<PathBuf>,
}

fn main() {
    let matches = App::new("nmkill")
        .arg(
            Arg::from_usage("-a --purge-all 'Remove all node_modules folders from the entire system'")
        )
        .arg(
            Arg::from_usage("<path> 'Remove node_modules folders from a specific directory'")
                .required_unless_one(&["purge-all"])
        )
        .get_matches();

    let cli = Cli {
        purge_all: matches.is_present("purge-all"),
        path: matches.value_of("path").map(PathBuf::from),
    };

    if cli.purge_all {
        prompt_and_purge_all_node_modules();
    } else if let Some(path) = cli.path {
        purge_node_modules_in_directory(&path);
    }
}

fn prompt_and_purge_all_node_modules() {
    println!("This operation will remove all node_modules folders from the entire system.");
    println!("Make sure you have the necessary permissions to perform this action.");
    print!("Are you sure you want to continue? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        purge_all_node_modules();
    } else {
        println!("Operation cancelled.");
    }
}

fn purge_all_node_modules() {
    println!("Removing all node_modules folders from the entire system...");
    // not done (lmao)
}

fn purge_node_modules_in_directory(path: &Path) {
    if !path.is_dir() {
        println!("Invalid directory path!");
        return;
    }

    println!("Removing node_modules folders in the directory: {:?}", path);
    let mut count = 0;
    visit_directories(path, &mut count);
    println!("Removed {} node_modules folders.", count);
}

fn visit_directories(path: &Path, count: &mut u32) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() && entry_path.ends_with("node_modules") {
                    if let Err(err) = fs::remove_dir_all(&entry_path) {
                        println!("Failed to remove directory: {:?}", err);
                    } else {
                        *count += 1;
                    }
                } else if entry_path.is_dir() {
                    visit_directories(&entry_path, count);
                }
            }
        }
    }
}
