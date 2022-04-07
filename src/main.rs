fn main() {
    use std::io::Write;
    use markdown::file_to_html;
    use html_editor::{parse, Selector, prelude::{Editable, Htmlifiable}};

    let args: Vec<String> = std::env::args().collect();
    let markdown_file = &args[1];
    let template_file = &args[2];
    let output_file = &args[3];

    // Read markdown file and convert to html, then simply read the template html file
    let markdown_html_contents = file_to_html(std::path::Path::new(markdown_file)).unwrap();
    let template_html_contents = std::fs::read_to_string(template_file).unwrap();

    let markdown_html = parse(&markdown_html_contents).unwrap();
    let mut template_html = parse(&template_html_contents).unwrap();
    

    for i in markdown_html {
        // Loop through every element in the markdown and add it to main
        template_html.insert_to(&Selector::from("main"), i);
    }

    let mut output = std::fs::File::create(output_file).unwrap();
    write!(output, "{}", template_html.trim().html());
}