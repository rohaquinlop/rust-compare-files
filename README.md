# compare-files

This a simple CLI tool to compare two files and show the differences.
It is written in Rust.

The tool is inspired by the `diff` command and `git` changes visualization,
but it is simpler and easier to use.
Always, the first file is the reference, and the second file is the one to compare.

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

The output will show the differences between the two files.
For example, if the files are:

```plaintext
file1.txt:
1
2
3
file2.txt:
1
3
4
```

The output will be:

```plaintext
1: 1
2: - 2
3: 3
3: + 4

The files you provided are differents!

Here is the summary:
+ 1 line added.
- 1 line deleted.
```

As you can see, the output shows the differences between both files.
If you're using an ansi-compatible terminal, you can see the output with colors.

Let's say you have two files, `file1.txt` and `file2.txt`, and you want
to compare them. You can run the following command:

```bash
compare-files file1.txt file2.txt
```

Even, you can compare them if they are in different directories:

```bash
compare-files ~/Documents/file1.txt ./file2.txt
```
