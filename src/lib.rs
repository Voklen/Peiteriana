mod output_path;

use std::path::PathBuf;
pub enum MyPath<'a> {
	PathBuf(PathBuf),
	Str(&'a str),
}

impl MyPath<'_> {
	pub fn to_path(&self) -> PathBuf {
		match self {
			MyPath::PathBuf(path) => path.to_owned(),
			MyPath::Str(str) => PathBuf::from(str)
		}
	}
}

impl std::fmt::Display for MyPath<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}

}

pub fn convert_dir(markdown_directory: &str, template_file: &str, output_directory: &str) {
	use rayon::prelude::*;
	let markdown_path = PathBuf::from(markdown_directory);

	// Send each file to convert as a job for the threads 
	let _: Vec<()> = files_in_dir_recursively(&markdown_path)
		.into_par_iter()
		.filter_map(|file| 
			match output_path::in_to_out_path(&file, markdown_directory, output_directory) {
				Some(output_file) => Some(convert(MyPath::PathBuf(file), template_file, &output_file)),
				None => None // If it's not a markdown file, skip it
			},
		).collect(); // Must .collect() because iterator adaptors are lazy and do nothing unless consumed
}

pub fn convert(markdown_file: MyPath, template_file: &str, output_file: &str) {
	use html_editor::{parse, prelude::{Editable, Htmlifiable}, Selector};
	use markdown::file_to_html;

	// Read markdown file and convert to html, then simply read the template html file
	let markdown_html_contents = file_to_html(&markdown_file.to_path())
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

	let output_path = PathBuf::from(output_file);
	if output_path.exists() {
		std::fs::write(output_file, template_html.trim().html())
			.unwrap_or_else(|err| {throw_error("write to", output_file, err.to_string());});
	} else {
		match output_path.parent() {
			Some(parent_dir) => {
				std::fs::create_dir_all(parent_dir)
					.unwrap_or_else(|err| {throw_error("create output directory for", output_file, err.to_string());});
				std::fs::write(output_file, template_html.trim().html())
					.unwrap_or_else(|err| {throw_error("create or write to", output_file, err.to_string());});
			},
			None => {}
		};
	}
}

fn throw_error<T: std::fmt::Display>(action: &str, file: T, err: String) -> (String, Vec<html_editor::Node>, std::fs::File) { 
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

fn files_in_dir_recursively(directory: &PathBuf) -> Vec<PathBuf> {
	std::fs::read_dir(directory).unwrap().flat_map(|i| {
		let file = i.unwrap().path();
		if file.is_dir() {
			return files_in_dir_recursively(&file)
		};
		if file.is_file() {
			return vec![file]
		};
		println!(
			"{program_name}: {file}: Could not read file or directory (maybe no read permission for this file?)",
			program_name = env!("CARGO_PKG_NAME"),
			file = file.display(),
		);
		std::process::exit(1)
	}).collect()
}