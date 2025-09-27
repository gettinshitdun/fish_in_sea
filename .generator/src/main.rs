mod file_handler;
mod markdown;
use std::{fs, path::Path};

use crate::file_handler::FileHandler;

fn main() {
    // delete the public directory for a fresh start
    let output_path = Path::new("../public");
    let _ = fs::remove_dir_all(output_path);

    // find all files in ../content and render them to ../public
    let mut file_paths: Vec<FileHandler> = Vec::new();
    handle_dir(Path::new("../content"), &mut file_paths);
    file_paths.iter().for_each(|handler| {
        handler.process_file();
    });
}

fn handle_dir(dir_path: &Path, files: &mut Vec<FileHandler>) {
    let dir_content = fs::read_dir(dir_path).unwrap();
    dir_content.for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_symlink() {
            return;
        }

        if path.is_dir() {
            handle_dir(&path, files);
        } else if path.is_file() {
            files.push(FileHandler::from_src_path(path));
        }
    });
}
