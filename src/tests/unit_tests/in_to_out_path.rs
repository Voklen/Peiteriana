use crate::*;
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
