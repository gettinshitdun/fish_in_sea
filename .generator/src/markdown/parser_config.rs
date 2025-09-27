use std::{collections::HashSet, hash::Hash};

use markdown_it::{MarkdownIt, plugins};

#[derive(Debug)]
pub(crate) struct MarkdownConfig {
    options: HashSet<MarkdownOption>,
}

impl MarkdownConfig {
    fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn add_option(&mut self, option: MarkdownOption) {
        self.options.insert(option);
    }

    pub fn get_parser(&self) -> MarkdownIt {
        let mut parser = MarkdownIt::new();
        for op in &self.options {
            match op {
                MarkdownOption::CMark => {
                    plugins::cmark::add(&mut parser);
                }
                MarkdownOption::Html => {
                    plugins::html::add(&mut parser);
                }
                MarkdownOption::StrikeThrough => {
                    plugins::extra::strikethrough::add(&mut parser);
                }
                MarkdownOption::Tables => {
                    plugins::extra::tables::add(&mut parser);
                }
                MarkdownOption::AutoLinkify => {
                    plugins::extra::linkify::add(&mut parser);
                }
                MarkdownOption::BeautifyLinks => {
                    plugins::extra::beautify_links::add(&mut parser);
                }
                MarkdownOption::HeadingAnchors => {
                    plugins::extra::heading_anchors::add(
                        &mut parser,
                        plugins::extra::heading_anchors::simple_slugify_fn,
                    );
                }
                MarkdownOption::Typography => {
                    plugins::extra::typographer::add(&mut parser);
                }
            }
        }

        return parser;
    }
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        let mut config = Self::new();
        config.add_option(MarkdownOption::Html);
        config.add_option(MarkdownOption::StrikeThrough);
        config.add_option(MarkdownOption::Tables);
        config.add_option(MarkdownOption::AutoLinkify);
        config.add_option(MarkdownOption::HeadingAnchors);
        return config;
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum MarkdownOption {
    CMark,
    Html,
    StrikeThrough,
    Tables,
    AutoLinkify,
    BeautifyLinks,
    HeadingAnchors,
    Typography,
}
