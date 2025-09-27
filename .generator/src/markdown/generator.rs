use std::{fs, path::PathBuf};

use crate::markdown::parser_config::MarkdownConfig;

#[derive(Debug, Default)]
pub struct MarkdownToHtmlGenerator {
    md_config: MarkdownConfig,
}

impl MarkdownToHtmlGenerator {
    pub fn get_html(&self, src_path: &PathBuf) -> String {
        let is_index = src_path.ends_with("index.md");

        // read file
        let md = fs::read_to_string(src_path);
        if let Err(err) = md {
            eprintln!(
                "Unable to read UTF-8 from file {:?}. Error: {}",
                src_path, err
            );
            return String::new();
        }

        // parse md -> html
        let parser = self.md_config.get_parser();
        let md = md.unwrap();
        let md_to_html = parser.parse(&md).render(); // this is the main content of the page

        return String::new();
    }

    fn get_title(src_path: &PathBuf) -> String {
        if let Some(parent) = src_path.parent() {
            let title = parent.file_name().expect("Directory name should exist!");
            return title.to_string_lossy().to_string();
        }
        return String::new();
    }
}
