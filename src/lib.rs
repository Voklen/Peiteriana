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
			MyPath::Str(str) => PathBuf::from(str),
		}
	}
}

impl std::fmt::Display for MyPath<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

pub struct ThrowError {
	failed_action: String,
	file: String,
	error: String,
}

impl std::fmt::Debug for ThrowError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{program_name}: Could not {action} file {file}: {error}",
			program_name = env!("CARGO_PKG_NAME"),
			action = self.failed_action,
			file = self.file,
			error = self.error
		)
	}
}

impl ThrowError {
	fn new<T: std::string::ToString, N: std::string::ToString>(
		one: &str,
		two: T,
		three: N,
	) -> Self {
		ThrowError {
			failed_action: one.to_string(),
			file: two.to_string(),
			error: three.to_string(),
		}
	}
}

pub fn convert_dir(
	markdown_directory: &str,
	template_file: &str,
	output_directory: &str,
) -> Result<(), ThrowError> {
	use rayon::prelude::*;
	let markdown_path = PathBuf::from(markdown_directory);

	// Send each file to convert as a job for the threads
	let yep: Option<ThrowError> = files_in_dir_recursively(&markdown_path)
		.into_par_iter()
		.find_map_any(|file| {
			let output_file =
				output_path::in_to_out_path(&file, markdown_directory, output_directory)?; // If it's not a markdown file, skip it
			match convert(MyPath::PathBuf(file), template_file, &output_file) {
				Ok(()) => None,
				Err(err) => Some(err),
			}
		});
	Ok(())
}

pub fn convert(
	markdown_file: MyPath,
	template_file: &str,
	output_file: &str,
) -> Result<(), ThrowError> {
	use html_editor::{
		parse,
		prelude::{Editable, Htmlifiable},
		Selector,
	};
	use markdown::file_to_html;

	// Read markdown file and convert to html, then simply read the template html file
	let markdown_html_contents = file_to_html(&markdown_file.to_path())
		.map_err(|err| ThrowError::new("open or parse markdown", markdown_file, err))?;
	let template_html_contents = std::fs::read_to_string(template_file)
		.map_err(|err| ThrowError::new("open HTML", template_file, err))?;

	let markdown_html = parse(&markdown_html_contents)
		.expect("The `markdown` and `html_editor` crates seem to have an incompatibility, please report this at https://github.com/Voklen/Peiteriana/issues with the markdown file used");
	let mut template_html = parse(&template_html_contents)
		.map_err(|err| ThrowError::new("parse", template_file, err))?;

	for i in markdown_html {
		// Loop through every element in the markdown and add it to main
		template_html.insert_to(&Selector::from("main"), i);
	}

	let output_path = PathBuf::from(output_file);
	if output_path.exists() {
		std::fs::write(output_file, template_html.trim().html())
			.map_err(|err| ThrowError::new("write to", output_file, err))?;
	} else {
		match output_path.parent() {
			Some(parent_dir) => {
				std::fs::create_dir_all(parent_dir)
					.map_err(|err| ThrowError::new("create output directory for", output_file, err))?;
				std::fs::write(output_file, template_html.trim().html())
					.map_err(|err| ThrowError::new("create or write to", output_file, err))?;
			}
			None => {}
		};
	};
	Ok(())
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
