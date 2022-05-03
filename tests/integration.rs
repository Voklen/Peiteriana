use peiteriana::*;
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
