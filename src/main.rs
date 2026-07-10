use clap::Parser;
use heic::DecoderConfig;
use std::fs;
use std::path::{Path, PathBuf};

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
    file_name: String,
}

fn convert_files(files: &Files) {
    for file in &files.files {
        if file.extension.to_lowercase() == "heic" {
            let data = fs::read(&file.path).unwrap();
            let output = DecoderConfig::new().decode_request(&data).decode().unwrap();
        }
    }
}

fn main() {
    let args = Cli::parse();

    let mut files = Files {
        files: Vec::<File>::new(),
        completed: false,
        to_extension: args.to,
    };

    for file in args.files.iter() {
        let path: PathBuf = PathBuf::from(file);
        let mut extension: String = String::new();
        if let Some(ext) = Path::new(file).extension() {
            if let Some(ext_str) = ext.to_str() {
                extension = String::from(ext_str);
            }
        }

        let mut file_name: String = String::new();
        if let Some(stem) = path.file_stem() {
            if let Some(stem_str) = stem.to_str() {
                file_name = String::from(stem_str);
            }
        }

        let curr_file = File {
            path: path,
            completed: false,
            extension: extension,
            file_name: file_name,
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
