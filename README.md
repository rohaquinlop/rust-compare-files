# rust-compare-files

This a simple CLI tool to compare two files and show the differences.
It is written in Rust.

## Installation

### From crates.io

To install the tool, you need to have [Rust](https://www.rust-lang.org/tools/install)
installed. Then, you can run the following command:

```bash
cargo install compare-files
```

### From source

or you can clone the repository:

```bash
git clone https://github.com/rohaquinlop/rust-compare-files.git
cd rust-compare-files
cargo install --path .
```

## Usage

To compare two files, you can run the following command:

```bash
compare-files /path/to/file1 /path/to/file2
```

Let's say you have two files, `file1.txt` and `file2.txt`, and you want
to compare them. You can run the following command:

```bash
compare-files file1.txt file2.txt
```

Even, you can compare them if they are in different directories:

```bash
compare-files ~/Documents/file1.txt ./file2.txt
```

If you're using a ansi-compatible terminal, you can see the output with colors.
