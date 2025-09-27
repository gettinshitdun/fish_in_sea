fn main() {

}

fn parse_markdown(md: &str) -> String {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);

    let ast = parser.parse(md);
    let html = ast.render();

    return html;
}
