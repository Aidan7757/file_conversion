use clap::Parser;
use heic::DecoderConfig;
use image::{ImageBuffer, ImageReader, Rgb};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    // #[arg(value_name = "DIRECTORY")]
    // directory: String,
    #[arg(short, long, value_name = "CONVERSION_TYPE")]
    to: String,
    // #[arg(short, long, value_name = "COMPRESSION")]
    // compression: bool,
}

#[derive(Debug)]
struct Files {
    files: Vec<File>,
    to_extension: String,
}

#[derive(Debug)]
struct File {
    path: std::path::PathBuf,
    extension: String,
    file_name: String,
}

fn convert_files(files: &mut Files) {
    for file in &mut files.files {
        if file.extension.to_lowercase() == "heic" {
            let data = fs::read(&file.path).unwrap();

            let output = DecoderConfig::new()
                .decode_request(&data)
                .with_output_layout(heic::PixelLayout::Rgb8)
                .decode()
                .unwrap();

            let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::from_raw(output.width, output.height, output.data)
                    .ok_or("Failed to convert image to image buffer.")
                    .unwrap();

            image::save_buffer(
                format!("{}.{}", file.file_name, files.to_extension),
                &img_buffer,
                output.width,
                output.height,
                image::ExtendedColorType::Rgb8,
            )
            .unwrap();
            println!("Saved new file: {}.{}", file.file_name, files.to_extension);
        } else {
            let data = fs::read(&file.path).unwrap();

            let dynamic_img = ImageReader::new(std::io::Cursor::new(&data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();

            let save_path = format!("{}.{}", file.file_name, files.to_extension);
            dynamic_img.save(&save_path).unwrap();
            println!("Saved new file: {}.{}", file.file_name, files.to_extension);
        }
    }
}

fn main() {
    let args = Cli::parse();

    let mut files = Files {
        files: Vec::<File>::new(),
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
    convert_files(&mut files);
    return;
}
