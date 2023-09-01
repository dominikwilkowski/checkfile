```sh
 ╔═╗ ╦ ╦ ╔═╗ ╔═╗ ╦╔═ ╔═╗ ╦ ╦   ╔═╗
 ║   ╠═╣ ║╣  ║   ╠╩╗ ╠╣  ║ ║   ║╣
 ╚═╝ ╩ ╩ ╚═╝ ╚═╝ ╩ ╩ ╚   ╩ ╩═╝ ╚═╝
```

<p align="center">
	<a href="https://crates.io/crates/checkfile"><img src="https://img.shields.io/crates/v/checkfile.svg" alt="checkfile badge"></a>
</p>

<p align="center">A command line tool that logs the checksum and last 50 lines of each file in a folder written in rust</p>

> I use this to verify some backblaze log files and to see how things are changing that may cause unwanted safety freezes.

## Install

You can install this tool via the crates cargo ecosystem:

```sh
λ cargo install checkfile
```

## Usage

You can `cd` into a folder and just run the tool with its defaults:

```sh
λ cd path/to/folder
λ checkfile
The log for 11 files was written successfully to ./checkfile.log
Finished in 1.30ms
```

This will create a file called `checkfile.log` that contains the name, checksum and last 50 lines of each file found in the current directory.

The format of the `checkfile.log` is as follows:

```
## NAME ./your_file.ext
## HASH 54d0aee5a905190551f607309d162ff7d970f845ac646da13469da004d8c8b63
-->
contents of file
[...]
contents of file
<--
## NAME ./your_file.ext
## HASH 54d0aee5a905190551f607309d162ff7d970f845ac646da13469da004d8c8b63
-->
contents of file
[...]
contents of file
<--
## NAME ./your_file.ext
## HASH 54d0aee5a905190551f607309d162ff7d970f845ac646da13469da004d8c8b63
-->
contents of file
[...]
contents of file
<--

```

### <PATH>
Type: `<path>`  
Default value: `.`

Set the directory you would like to run `checkfile` on.

```sh
λ checkfile /path/to/folder
```

### -o, --output <PATH>
Type: `<path>`  
Default value: `./checkfile.log`

Set this output file name and path.

```sh
λ checkfile -o /path/to/folder/yourfile.log
```

_💡 Note: This will not create non-existing folder so the path has to exist_

### -l, --line <NUMBER>
Type: `<number>`  
Default value: `50`

Set this amount of lines that should be included into the log file for each checked file.

```sh
λ checkfile -l 10
```

## License
Copyleft (c) 2023 Dominik Wilkowski.
Licensed under the [GNU GPL-3.0-or-later](https://github.com/dominikwilkowski/checkfile/blob/main/LICENSE).
