use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(required = true, value_name = "FILE")]
    files: Vec<PathBuf>,

    #[arg(required = true, short, long, value_name = "CONVERSION_TYPE")]
    to: String,
}

#[derive(Debug)]
struct Files {
    files: Vec<File>,
    completed: bool,
    to_extension: String,
}

#[derive(Debug, Default)]
struct File {
    path: std::path::PathBuf,
    completed: bool,
    extension: String,
}

fn convert_files(files: &Files) {}

fn main() {
    let args = Cli::parse();

    let mut files = Files {
        files: Vec::<File>::new(),
        completed: false,
        to_extension: args.to,
    };

    for file in args.files.iter() {
        let mut extension: String = String::new();
        if let Some(ext) = std::path::Path::new(file).extension() {
            if let Some(ext_str) = ext.to_str() {
                extension = String::from(ext_str);
            }
        }
        let curr_file = File {
            path: std::path::PathBuf::from(file),
            completed: false,
            extension: extension,
        };
        if !curr_file.path.exists() {
            eprintln!(
                "Path: {:?} does not existing within the CWD.",
                curr_file.path
            );
            continue;
        }
        files.files.push(curr_file);
    }
    println!("All collected files: {:?}", files);
    return;
}
