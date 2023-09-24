use std::sync::Mutex;

use clap::{Parser, Subcommand};
use rayon::prelude::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GitUpdateAll { dir } => {
            git_update_all(dir);
        }
        Commands::HtmlToMd { file } => {
            html_to_md(file);
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    #[command(
        name = "git-update-all",
        about = "Update all git repositories in a directory"
    )]
    GitUpdateAll { dir: String },

    #[command(name = "html-to-md", about = "Convert a html file to markdown")]
    HtmlToMd { file: Vec<String> },
}

fn git_update_all(dir: &String) {
    let dir = std::path::Path::new(dir);

    println!("Updating all git repos in {}", dir.display());

    let mut dirs = vec![];

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() && path.join(".git").exists() {
            dirs.push(path);
        }
    }

    println!("Found {} git repositories\n", dirs.len());

    let write_lock = Mutex::new(());

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

fn html_to_md(files: &Vec<String>) {
    for file in files {
        html_to_md_file(file);
    }
}

fn html_to_md_file(file: &String) {
    println!("Converting {} to markdown", file);

    std::process::Command::new("pandoc")
        .arg(file)
        .arg("-f")
        .arg("html")
        .arg("-t")
        .arg("commonmark")
        .arg("-o")
        .arg(file.replace(".html", ".md"))
        .output()
        .expect("failed to execute pandoc");
}
