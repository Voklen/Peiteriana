fn main() {
	let args: Vec<String> = std::env::args().collect();
	let markdown_file = &args[1];
	let template_file = &args[2];
	let output_file = &args[3];

	convert(markdown_file, template_file, output_file);
}

fn convert(markdown_file: &str, template_file: &str, output_file: &str) {
	use html_editor::{parse, prelude::{Editable, Htmlifiable}, Selector};
	use markdown::file_to_html;
	use std::io::Write;

	// Read markdown file and convert to html, then simply read the template html file
	let markdown_html_contents = file_to_html(std::path::Path::new(markdown_file))
		.unwrap_or_else(|err| throw_error("open or parse markdown", markdown_file, err.to_string()).0);
	let template_html_contents = std::fs::read_to_string(template_file)
		.unwrap_or_else(|err| throw_error("open HTML", template_file, err.to_string()).0);

	let markdown_html = parse(&markdown_html_contents)
		.expect("The `markdown` and `html_editor` crates seem to have an incompatibility, please report this at https://github.com/Voklen/Peiteriana/issues with the markdown file used");
	let mut template_html = parse(&template_html_contents)
		.unwrap_or_else(|err| throw_error("parse", template_file, err).1);

	for i in markdown_html {
		// Loop through every element in the markdown and add it to main
		template_html.insert_to(&Selector::from("main"), i);
	}

	let mut output = std::fs::File::create(output_file)
		.unwrap_or_else(|err| throw_error("open or create output", output_file, err.to_string()).2);
	write!(output, "{}", template_html.trim().html())
		.unwrap_or_else(|err| {throw_error("write to output", output_file, err.to_string());});
}

fn throw_error(action: &str, file: &str, err: String) -> (String, Vec<html_editor::Node>, std::fs::File) { 
	// All these return types are just to be able to put it in a unwrap_or_else by just indexing the tuple for the type
	println!(
		"{program_name}: Could not {action} file {file}: {error}",
		program_name = env!("CARGO_PKG_NAME"),
		action = action,
		file = file,
		error = err
	);
	std::process::exit(1)
}

#[cfg(test)]
mod tests {
	#[test]
	fn integration_test() {
		// Imports and definitions
		use std::fs;
		let markdown_file = "test_data/test.md";
		let template_file = "test_data/template.html";
		let output_file = "test_data/output/integration_test.html";
		let expected_result = "test_data/expected.html";

		// Actual test code
		crate::convert(markdown_file, template_file, output_file);
		assert_eq!(
			fs::read_to_string(output_file).unwrap(), 
			fs::read_to_string(expected_result).unwrap()
		);

		// Clean up after test
		fs::remove_file(output_file).unwrap();
	}
}
