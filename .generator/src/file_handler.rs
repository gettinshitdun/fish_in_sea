use std::{
    fmt::Display,
    fs::{self, File},
    path::PathBuf,
};

use crate::markdown::generator::MarkdownToHtmlGenerator;

#[derive(Debug)]
pub struct FileHandler {
    html_generator: MarkdownToHtmlGenerator,
    src_path: PathBuf,
    dest_path: PathBuf,
}

impl FileHandler {
    pub fn from_src_path(src_path: PathBuf) -> Self {
        let html_generator = MarkdownToHtmlGenerator::default();

        let dest_path = Self::get_clean_html_path(&src_path);

        return FileHandler {
            dest_path,
            src_path,
            html_generator,
        };
    }

    pub fn process_file(&self) {
        // parse md file to html
        if self.src_path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let html = self.html_generator.get_html(&self.src_path);

            // create the destination file
            if let Some(parent) = self.dest_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            File::create_new(&self.dest_path).unwrap();

            // write to destination file
            fs::write(&self.dest_path, html).unwrap();
        }

        // simply copy the css file
        if self.src_path.extension().and_then(|ext| ext.to_str()) == Some("css") {
            if let Some(parent) = self.dest_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&self.src_path, &self.dest_path).unwrap();
        }
    }

    fn get_clean_html_path(src_path: &PathBuf) -> PathBuf {
        let path = src_path.to_str().unwrap();
        let direct_path = path.strip_prefix("../content").unwrap();
        let mut dest_path = String::from("../public") + direct_path;
        dest_path.make_ascii_lowercase();
        let mut dest_path = dest_path.replace(" ", "-");
        if dest_path.contains(|c: char| {
            return !(c.is_alphanumeric() || c == '/' || c == '.' || c == '-');
        }) {
            panic!(
                "File {} contains illegal characters, destination path: {}",
                path.strip_prefix("../").unwrap(),
                dest_path
            );
        }

        if dest_path.ends_with("md") {
            dest_path = String::from(dest_path.strip_suffix("md").unwrap()) + "html";
        }

        return PathBuf::from(dest_path);
    }
}

impl Display for FileHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source Path: {:?}
Destination Path: {:?}\n",
            self.src_path.to_str(),
            self.dest_path.to_str(),
        )
    }
}
