use clap::{crate_version, Command};
use rev_buf_reader::RevBufReader;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

fn main() -> Result<(), io::Error> {
	let matches = Command::new("checkfile")
		.about("A command line tool that logs the checksum and last 50 lines of each file in a folder")
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
		.get_matches();

	let user_path = matches.get_one::<PathBuf>("dir").unwrap();
	let user_lines = matches.get_one::<usize>("lines").unwrap();
	let user_output = matches.get_one::<PathBuf>("output").unwrap();
	let mut contents = String::from("");

	let files = fs::read_dir(user_path)?
		.filter(|r| r.is_ok())
		.map(|r| r.unwrap().path())
		.filter(|r| !r.is_dir())
		.collect::<Vec<PathBuf>>();

	let files_count = files.len();

	println!("Processing files\n");

	for (i, this_path) in files.iter().enumerate() {
		println!("\x1b[2k\x1b[1F {i} of {files_count}");
		let bytes = std::fs::read(this_path).unwrap();
		let hash = sha256::digest(&bytes);
		contents.push_str(&format!("## NAME {}\n## HASH {hash}\n", this_path.display()));

		let file = fs::File::open(this_path);
		let lines = lines_from_file(&file.unwrap(), *user_lines);
		contents.push_str(&format!("-->\n{}\n<--\n", lines));
	}

	fs::write(user_output, contents)?;
	println!(
		"\x1b[2F\x1b[0K\n\x1b[2kThe log for {files_count} files was written successfully to {}",
		user_output.display()
	);
	Ok(())
}

fn lines_from_file(file: &fs::File, limit: usize) -> String {
	let buf = RevBufReader::new(file);
	buf.lines().take(limit).map(|l| l.expect("Could not parse line")).collect::<Vec<String>>().join("\n")
}
