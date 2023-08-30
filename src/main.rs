use rev_buf_reader::RevBufReader;
use std::env::args;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

fn main() -> Result<(), io::Error> {
	let args = args().collect::<Vec<String>>();
	let default = &String::from(".");
	let user_path = args.get(1).unwrap_or(default);
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
		let lines = lines_from_file(&file.unwrap(), 50);
		contents.push_str(&format!("-->\n{}\n<--\n", lines));
	}

	fs::write("checkfile.log", contents)?;
	println!("\x1b[2F\x1b[0K\n\x1b[2kLogs for {files_count} files written successfully");
	Ok(())
}

fn lines_from_file(file: &fs::File, limit: usize) -> String {
	let buf = RevBufReader::new(file);
	buf.lines().take(limit).map(|l| l.expect("Could not parse line")).collect::<Vec<String>>().join("\n")
}
