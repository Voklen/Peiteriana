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

#[derive(Debug)]
pub struct Error {
	failed_action: String,
	file: String,
	error: String,
}

impl std::fmt::Display for Error {
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

impl Error {
	fn new<T: std::string::ToString, N: std::string::ToString>(
		failed_action: &str,
		file: T,
		error: N,
	) -> Self {
		Error {
			failed_action: failed_action.to_string(),
			file: file.to_string(),
			error: error.to_string(),
		}
	}
}

pub fn convert_dir(
	markdown_directory: &str,
	template_file: &str,
	output_directory: &str,
) -> Result<(), Error> {
	use rayon::prelude::*;
	let markdown_path = PathBuf::from(markdown_directory);

	// Add each file conversion to the thread pool
	let conversion_result: Option<Error> = files_in_dir_recursively(&markdown_path)
		.into_par_iter()
		.find_map_any(|file| {
			// The `?` at the end of the line means if it's not a markdown file, skip it
			let output_file = output_path::in_to_out_path(&file, markdown_directory, output_directory)?;
			match convert(MyPath::PathBuf(file), template_file, &output_file) {
				Ok(()) => None,
				Err(err) => Some(err),
			}
		});
	match conversion_result {
		None => Ok(()),
		Some(err) => Err(err),
	}
}

pub fn convert(
	markdown_file: MyPath,
	template_file: &str,
	output_file: &str,
) -> Result<(), Error> {
	use html_editor::{parse, prelude::{Editable, Htmlifiable}, Selector};
	use markdown::file_to_html;

	// Read markdown file and convert to html, then simply read the template html file
	let markdown_html_contents = file_to_html(&markdown_file.to_path())
		.map_err(|err| Error::new("open or parse markdown", markdown_file, err))?;
	let template_html_contents = std::fs::read_to_string(template_file)
		.map_err(|err| Error::new("open HTML", template_file, err))?;

	let markdown_html = parse(&markdown_html_contents)
		.expect("The `markdown` and `html_editor` crates seem to have an incompatibility, please report this at https://github.com/Voklen/Peiteriana/issues with the markdown file used");
	let mut template_html = parse(&template_html_contents)
		.map_err(|err| Error::new("parse", template_file, err))?;

	for i in markdown_html {
		// Loop through every element in the markdown and add it to main
		template_html.insert_to(&Selector::from("main"), i);
	}

	let output_path = PathBuf::from(output_file);
	if output_path.exists() {
		std::fs::write(output_file, template_html.trim().html())
			.map_err(|err| Error::new("write to", output_file, err))?;
	} else {
		match output_path.parent() {
			Some(parent_dir) => {
				std::fs::create_dir_all(parent_dir)
					.map_err(|err| Error::new("create output directory for", output_file, err))?;
				std::fs::write(output_file, template_html.trim().html())
					.map_err(|err| Error::new("create or write to", output_file, err))?;
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
