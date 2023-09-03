use clap::{crate_version, Command};
use rev_lines::RevLines;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), io::Error> {
	let now = Instant::now();

	let matches = Command::new("checkfile")
		.about(" ╔═╗ ╦ ╦ ╔═╗ ╔═╗ ╦╔═ ╔═╗ ╦ ╦   ╔═╗\n ║   ╠═╣ ║╣  ║   ╠╩╗ ╠╣  ║ ║   ║╣\n ╚═╝ ╩ ╩ ╚═╝ ╚═╝ ╩ ╩ ╚   ╩ ╩═╝ ╚═╝\n\nA command line tool that logs the name, checksum and last 50 lines of each file of a folder in a log file.")
		.author("Dominik Wilkowski <hi@dominik-wilkowski.com>")
		.version(crate_version!())
		.arg(
			clap::arg!([dir] "Set the directory you want to check")
				.required(false)
				.value_parser(clap::value_parser!(PathBuf))
				.default_value("."),
		)
		.arg(
			clap::arg!(-l --lines <VALUE> "Set the lines of each")
				.required(false)
				.value_parser(clap::value_parser!(usize))
				.default_value("50"),
		)
		.arg(
			clap::arg!(-o --output <FILE> "Set the output file")
				.required(false)
				.value_parser(clap::value_parser!(PathBuf))
				.default_value("./checkfile.log"),
		)
		.arg(
			clap::arg!(-d --dotfiles "Include dot files in the output")
				.required(false),
		)
		.get_matches();

	let user_path = matches.get_one::<PathBuf>("dir").unwrap();
	let user_lines = matches.get_one::<usize>("lines").unwrap();
	let user_output = matches.get_one::<PathBuf>("output").unwrap();
	let user_dotfiles = matches.get_one::<bool>("dotfiles").unwrap();
	let mut contents = String::from("");

	let files = fs::read_dir(user_path)?
		.filter(|r| r.is_ok())
		.map(|r| r.unwrap().path())
		.filter(|r| !r.is_dir() && !r.to_str().unwrap().starts_with('.') || !r.is_dir() && *user_dotfiles)
		.collect::<Vec<PathBuf>>();

	let files_count = files.len();

	println!("Processing files\n");

	for (i, this_path) in files.iter().enumerate() {
		println!("\x1b[2k\x1b[1F {i} of {files_count}");
		let bytes = std::fs::read(this_path).unwrap();
		let hash = sha256::digest(&bytes);
		contents.push_str(&format!("## NAME {}\n## HASH {hash}\n", this_path.display()));

		let file = fs::File::open(this_path)?;
		let lines = lines_from_file(&file, *user_lines);
		contents.push_str(&format!("-->\n{}\n<--\n", lines));
	}

	fs::write(user_output, contents)?;
	println!(
		"\x1b[2F\x1b[0KThe log for {files_count} files was written successfully to {}\n\x1b[2kFinished in {:.2?}",
		user_output.display(),
		now.elapsed()
	);
	Ok(())
}

fn lines_from_file(file: &fs::File, limit: usize) -> String {
	let reader = RevLines::new(file);
	reader
		.take(limit)
		.map(|line| match line {
			Ok(this_line) => this_line,
			Err(_) => String::from("[- binary data -]"),
		})
		.collect::<Vec<String>>()
		.join("\n")
}
