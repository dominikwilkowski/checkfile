use chrono::Utc;
use clap::{crate_version, Command};
use crossterm::tty::IsTty;
use rev_lines::RevLines;
use std::fs;
use std::io;
use std::io::stdout;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), io::Error> {
	let now = Instant::now();

	let matches = Command::new("checkfile")
		.about(
			" ╔═╗ ╦ ╦ ╔═╗ ╔═╗ ╦╔═ ╔═╗ ╦ ╦   ╔═╗\n \
			  ║   ╠═╣ ║╣  ║   ╠╩╗ ╠╣  ║ ║   ║╣\n \
			  ╚═╝ ╩ ╩ ╚═╝ ╚═╝ ╩ ╩ ╚   ╩ ╩═╝ ╚═╝\n\n\
			A command line tool that writes the name, \
			checksum and last 50 lines of each file of a folder in a log file.",
		)
		.author("Dominik Wilkowski <hi@dominik-wilkowski.com>")
		.version(crate_version!())
		.arg(
			clap::arg!([dir] "Set the directory you want to check")
				.required(false)
				.value_parser(clap::value_parser!(PathBuf))
				.default_value("."),
		)
		.arg(
			clap::arg!(-o --output <FILE> "Set the output file")
				.required(false)
				.value_parser(clap::value_parser!(PathBuf))
				.default_value("./checkfile.log"),
		)
		.arg(
			clap::arg!(-l --lines <VALUE> "Set the lines of each")
				.required(false)
				.value_parser(clap::value_parser!(usize))
				.default_value("50"),
		)
		.arg(clap::arg!(-d --dotfiles "Include dot files in the output").required(false))
		.arg(clap::arg!(-r --reverse "Reverse the output lines").required(false))
		.get_matches();

	let is_tty: bool = stdout().is_tty();

	let user_path = matches.get_one::<PathBuf>("dir").unwrap();
	let user_lines = matches.get_one::<usize>("lines").unwrap();
	let user_output = matches.get_one::<PathBuf>("output").unwrap();
	let user_dotfiles = matches.get_one::<bool>("dotfiles").unwrap();
	let user_reverse = matches.get_one::<bool>("reverse").unwrap();
	let mut contents = String::from("");

	let files = fs::read_dir(user_path)?
		.filter(|r| r.is_ok())
		.map(|r| r.unwrap().path())
		.filter(|r| {
			!r.is_dir() && !r.file_name().unwrap().to_str().unwrap().starts_with('.') || !r.is_dir() && *user_dotfiles
		})
		.collect::<Vec<PathBuf>>();

	let files_count = files.len();

	if is_tty {
		println!("Processing files\n");
	}

	for (i, this_path) in files.iter().enumerate() {
		if is_tty {
			println!("\x1b[2k\x1b[1F {i} of {files_count}");
		}
		let bytes = std::fs::read(this_path).unwrap();
		let hash = sha256::digest(&bytes);
		contents.push_str(&format!("## NAME {}\n## HASH {hash}\n", this_path.display()));

		let file = fs::File::open(this_path)?;
		let lines = lines_from_file(&file, *user_lines, *user_reverse);
		contents.push_str(&format!("-->\n{}\n<--\n", lines));
	}

	fs::write(user_output, contents)?;
	if is_tty {
		println!(
			"\x1b[2F\x1b[0KThe log for {files_count} files was written successfully to {}\n\x1b[2kFinished in {:.2?}",
			user_output.display(),
			now.elapsed()
		);
	} else {
		println!(
			"[OK] [{}] - written {files_count} files to {} in {:.2?}",
			Utc::now().to_rfc3339(),
			user_output.display(),
			now.elapsed()
		);
	}
	Ok(())
}

fn lines_from_file(file: &fs::File, user_lines: usize, user_reverse: bool) -> String {
	let reader = RevLines::new(file);
	let mut output = reader
		.take(user_lines)
		.map(|line| match line {
			Ok(this_line) => this_line,
			Err(_) => String::from("[- binary data -]"),
		})
		.collect::<Vec<String>>();

	if user_reverse {
		let length = output.len() - 1;
		output[0..=length].reverse();
		output = output.to_vec();
	}

	output.join("\n")
}
