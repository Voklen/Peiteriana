use peiteriana::*;

#[test]
fn normal() {
	// Imports and definitions
	use std::fs;
	let markdown_file = "test_data/test.md";
	let template_file = "test_data/template.html";
	let output_file = "test_data/output/integration_test.html";
	let expected_result = "test_data/expected.html";

	// Actual test code
	convert(MyPath::Str(markdown_file), template_file, output_file).unwrap();
	assert_eq!(
		fs::read_to_string(output_file).unwrap(),
		fs::read_to_string(expected_result).unwrap()
	);

	// Clean up after test
	fs::remove_file(output_file).unwrap();
}

#[test]
fn wrong_markdown_file_name() {
	// Imports and definitions
	let markdown_file = "test_data/tes.md";
	let template_file = "test_data/template.html";
	let output_file = "test_data/output/integration_test.html";

	// Actual test code
	let result = convert(MyPath::Str(markdown_file), template_file, output_file);
	let err = result.unwrap_err();
	assert_eq!(err.file, markdown_file);
	assert_eq!(err.error, "No such file or directory (os error 2)");
}
