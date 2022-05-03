use peiteriana::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let markdown_file = &args[1];
	let template_file = &args[2];
	let output_file = &args[3];

	let markdown_is_file = is_file(markdown_file);
	let template_is_file = is_file(template_file);

	match (markdown_is_file, template_is_file) {
		(true, true) => convert(MyPath::Str(markdown_file), template_file, output_file),
		(true, false) => {
			println!(
				"{}: Cannot have markdown file and template directory",
				env!("CARGO_PKG_NAME")
			);
			std::process::exit(1)
		}
		(false, true) => convert_dir(markdown_file, template_file, output_file),
		_ => panic!(),
	};
}

fn is_file(file_as_str: &str) -> bool {
	let file = std::path::Path::new(file_as_str);
	if !file.exists() {
		println!(
			"{program_name}: {file}: No such file or directory",
			program_name = env!("CARGO_PKG_NAME"),
			file = file_as_str,
		);
		std::process::exit(1)
	}
	if file.is_file() {
		return true;
	}
	if file.is_dir() {
		return false;
	}
	println!(
		"{program_name}: {file}: Could not read file or directory (maybe no read permission for this file?)",
		program_name = env!("CARGO_PKG_NAME"),
		file = file_as_str,
	);
	std::process::exit(1)
}
