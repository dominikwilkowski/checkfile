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

This will create a file called `checkfile.log` that contains the name, checksum and last 50 lines _(in reverse order)_ of each file found in the current directory.

The format of the `checkfile.log` is as follows:

```
## NAME ./your_file1.ext
## HASH 54d0aee5a905190551f607309d162ff7d970f845ac646da13469da004d8c8b63
-->
last line
second last line
third last line
[...]
contents of file
<--
## NAME ./your_file2.ext
## HASH 895344059424ed7f5b1946d80c0e2581e30fc2032584db6dc36c608849e6f165
-->
contents of file
[...]
contents of file
<--
## NAME ./your_file3.ext
## HASH 104f62f4e75447518c3de21b9de71e757c0b7d719de77d36e81a024394777a53
-->
contents of file
[...]
contents of file
<--
```

## Scheduling

I use [`crontab`](https://www.man7.org/linux/man-pages/man5/crontab.5.html) to schedule `checkfile` to run every 6 hours and add the log to a folder.
If something is wrong with the backblaze backup I can go into that folder and introspect what may have happened.

```sh
λ crontab -l

0 */6 * * * checkfile -o ~/Desktop/logs/checkfile-$(date +%s).log /Library/Backblaze.bzpkg/bzdata/bzbackup/bzdatacenter >> ~/Desktop/logs/error.log 
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

### -d, --dotfiles
Type: `<bool>`  
Default value: `false`

Set to include dot files in your log.

```sh
λ checkfile -d
```

_💡 Note: If checkfiles encounters a binary file it will try to read it and mark lines it couldn't with `[- binary data -]`_

### -r, --reverse
Type: `<bool>`  
Default value: `false`

Reverse the output lines so they look the same way the files look (mirror what `tail` does)

```sh
λ checkfile -r
```

The output file without the `--reverse` flag:

```
## NAME ./your_file.ext
## HASH 54d0aee5a905190551f607309d162ff7d970f845ac646da13469da004d8c8b63
-->
last line
second last line
third last line
<--
```

The output file with the `--reverse` flag:

```
## NAME ./your_file.ext
## HASH 54d0aee5a905190551f607309d162ff7d970f845ac646da13469da004d8c8b63
-->
third last line
second last line
last line
<--
```

For comparison, the contents of `./your_file.ext`:

```
third last line
second last line
last line
```

## License
Copyleft (c) 2023 Dominik Wilkowski.
Licensed under the [GNU GPL-3.0-or-later](https://github.com/dominikwilkowski/checkfile/blob/main/LICENSE).
