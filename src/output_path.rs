use std::path::PathBuf;
// See tests for examples of how this works
pub fn in_to_out_path(file: &PathBuf, markdown_dir: &str, output_dir: &str) -> Option<String> {
	if file.extension() != Some(std::ffi::OsStr::new("md")) {
		return None;
	};
	let file = file.with_extension("html");
	let markdown_dir = with_slash_ending(markdown_dir);
	let mut output_dir = with_slash_ending(output_dir);

	// WARNING: This should be fine to just return a None to ignore a non-unicode file. 
	// But this _could_ result in a bug somewhere so I'm adding this comment to make finding it easier
	let file = file.to_str()?;
	
	// Slices like these can panic because of the varying number of bytes of unicode characters
	// This should never panic because `markdown_dir` should always be at the start of `file`
	// If it isn't then something's gone very wrong so we have no idea what state the program's in and so a panic is appropriate
	let path_relative_to_input_dir = &file[markdown_dir.len()..]; 
	// add that onto the last section
	output_dir.push_str(path_relative_to_input_dir);
	Some(output_dir)
}

fn with_slash_ending(directory: &str) -> String {
	if directory.ends_with("/") {
		directory.to_owned()
	} else {
		format!("{}/", directory)
	}
}

#[test]
fn normal() {
	let test_data = [
		(
			"input/blog1.md",
			"input/",
			"data/output/",
			"data/output/blog1.html",
		),
		(
			"input/hello/other/first_section/blog2.md",
			"input/hello/other/",
			"other_data/output/my_blog/",
			"other_data/output/my_blog/first_section/blog2.html",
		),
		(
			"an_input/a_markdown_file.md",
			"an_input/",
			"an_output/",
			"an_output/a_markdown_file.html",
		),
	];
	for i in test_data {
		let output = in_to_out_path(&PathBuf::from(i.0), i.1, i.2);
		assert_eq!(output, Some(i.3.to_owned()))
	}
}
#[test]
fn spaces() {
	let test_data = [
		(
			"input/blog 1.md",
			"input/",
			"data/output/",
			"data/output/blog 1.html",
		),
		(
			"input/hello/other/first section/blog2.md",
			"input/hello/other/",
			"other data/output/my_blog/",
			"other data/output/my_blog/first section/blog2.html",
		),
		(
			"an input / a markdown file.md",
			"an input /",
			"an output /",
			"an output / a markdown file.html",
		),
	];
	for i in test_data {
		let output = in_to_out_path(&PathBuf::from(i.0), i.1, i.2);
		assert_eq!(output, Some(i.3.to_owned()))
	}
}
#[test]
fn no_slash() {
	let test_data = [
		(
			"input/blog1.md",
			"input",
			"data/output",
			"data/output/blog1.html",
		),
		(
			"input/hello/other/first_section/blog2.md",
			"input/hello/other/",
			"other_data/output/my_blog",
			"other_data/output/my_blog/first_section/blog2.html",
		),
		(
			"an_input/a_markdown_file.md",
			"an_input/",
			"an_output/",
			"an_output/a_markdown_file.html",
		),
	];
	for i in test_data {
		let output = in_to_out_path(&PathBuf::from(i.0), i.1, i.2);
		assert_eq!(output, Some(i.3.to_owned()))
	}
}
#[test]
fn filter_non_markdown() {
	let test_data = [
		("input/blog1.html", "input", "data/output"),
		(
			"input/hello/other/first_section/blog2.mdx",
			"input/hello/other/",
			"other_data/output/my_blog",
		),
		("an_input/a_markdown_file.cmd", "an_input/", "an_output/"),
	];
	for i in test_data {
		let output = in_to_out_path(&PathBuf::from(i.0), i.1, i.2);
		assert_eq!(output, None)
	}
}
#[test]
fn unicode() {
	let test_data = [
		(
			"input/блог 1.md",
			"input",
			"данные/output",
			"данные/output/блог 1.html",
		),
		(
			"input/👋/other/first_section/💕.md",
			"input/👋/other/",
			"other_data/output/🎷🐛/",
			"other_data/output/🎷🐛/first_section/💕.html",
		),
		(
			"Mowię po polsku/انا لا اعرف.md",
			"Mowię po polsku/",
			"слава україні🇺🇦/",
			"слава україні🇺🇦/انا لا اعرف.html",
		),
	];
	for i in test_data {
		let output = in_to_out_path(&PathBuf::from(i.0), i.1, i.2);
		assert_eq!(output, Some(i.3.to_owned()))
	}
}
