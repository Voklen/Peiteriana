mod integration_tests {
	use crate::*;
	#[test]
	fn integration_test() {
		// Imports and definitions
		use std::fs;
		let markdown_file = "test_data/test.md";
		let template_file = "test_data/template.html";
		let output_file = "test_data/output/integration_test.html";
		let expected_result = "test_data/expected.html";

		// Actual test code
		convert(MyPath::Str(markdown_file), template_file, output_file);
		assert_eq!(
			fs::read_to_string(output_file).unwrap(),
			fs::read_to_string(expected_result).unwrap()
		);

		// Clean up after test
		fs::remove_file(output_file).unwrap();
	}
	#[test]
	fn dir_integration_test() {
		// Imports and definitions
		use dir_assert::assert_paths;
		use std::fs;
		let markdown_dir = "test_data/dir test input/";
		let template_file = "test_data/template.html";
		let output_dir = "test_data/output/dir_integration_test";
		let expected_result = "test_data/dir_integration_expected/";

		// Actual test code
		convert_dir(markdown_dir, template_file, output_dir);
		assert_paths!(output_dir, expected_result);

		// Clean up after test
		fs::remove_dir_all(output_dir).unwrap();
	}
}
mod in_to_out_path_unit_tests {
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
}
