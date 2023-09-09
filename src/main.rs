use std::sync::{Arc, Mutex};

use clap::{Parser, Subcommand};
use rayon::prelude::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::GitUpdateAll { dir }) => {
            git_update_all(dir);
        }
        None => {
            eprintln!("No command specified. Use --help to see available commands.");
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
enum Commands {
    GitUpdateAll { dir: String },
}

fn git_update_all(dir: &String) {
    println!("Updating all git repos in {}", dir);

    let dir = std::path::Path::new(dir);

    let mut dirs = vec![];

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() && path.join(".git").exists() {
            dirs.push(path);
        }
    }

    println!("Found {} git repositories", dirs.len());

    let write_lock = Arc::new(Mutex::new(()));

    dirs.par_iter().for_each(|dir| {
        let dir = dir.to_str().unwrap();

        let output = std::process::Command::new("git")
            .arg("pull")
            .current_dir(dir)
            .output()
            .expect("failed to execute process");

        let _lock = write_lock.lock().unwrap();

        println!("Updating {}", dir);
        println!("{}", String::from_utf8(output.stdout).unwrap());
    });
}
