use peiteriana::*;

#[test]
fn normal() {
	// Imports and definitions
	use dir_assert::assert_paths;
	use std::fs;
	let markdown_dir = "test_data/dir test input/";
	let template_file = "test_data/template.html";
	let output_dir = "test_data/output/dir_integration_test";
	let expected_result = "test_data/dir_integration_expected/";

	// Actual test code
	convert_dir(markdown_dir, template_file, output_dir).unwrap();
	assert_paths!(output_dir, expected_result);

	// Clean up after test
	fs::remove_dir_all(output_dir).unwrap();
}
