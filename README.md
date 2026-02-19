# Rep

Command-line find and replace with a text editor

![Rep](rep.png)

`rep` is a command-line utility that takes [`grep`](https://en.wikipedia.org/wiki/Grep)-formatted lines (e.g., from [`ripgrep`](https://github.com/BurntSushi/ripgrep)) and applies a find and replace on the matches. It does basic find and replace right on the command line, or the `grep` output can be edited manually with a text editor, and then the changes can be applied.

This means you can use all the features of a text editor including multiple cursors, find and replace, sorting, etc., and then apply the changes. For example:

1. `rg -n "(BananaStand|banana[_-]stand)" > /tmp/out.txt`
2. Edit `/tmp/out.txt` in a text editor turning banana stand into lemonade stand
3. `rep < /tmp/out.txt` to review the changes as a diff
4. `rep -w < /tmp/out.txt` to write the changes

## Simple Example

`rep` can also perform a find and replace from `grep` output without using a text editor.

[![Find & replace with `rep`](demo.gif)](https://www.youtube.com/embed/QIOKKTnC9-I)

Output a diff to standard output replacing `foo` with `bar`:

```
grep -n foo * | rep foo bar
```

Add the `-w` flag to write the changes to the files in place:

```
grep -n foo * | rep foo bar -w
```

The `-n` (`--line-number`) option is required so that `grep` outputs the line number for each match.

Use `--` to separate options from arguments when the pattern starts with a hyphen:

```
grep -n -- --foo * | rep -- '--foo' '--bar'
```

## Writing by Editing Standard Input

Like [`wgrep`](https://github.com/mhayashi1120/Emacs-wgrep) for Emacs, writing to files can also be accomplished by editing the contents of the `grep` output itself (and omitting the find and replace arguments).

This means for example a workflow like this will work:

1. `grep -n foo * > tmp`: Save the `grep` matches to a file named `tmp`.
2. `sed -i '' s/foo/bar/g tmp`: Replace `foo` with `bar` in the `tmp` file.
3. `rep < tmp`: Display a diff of replacing `foo` with `bar` for each `grep` match.
4. `rep -w < tmp`: Write the changes to replace `foo` with `bar` for each `grep` match.

## Flow

The flow `rep` uses when making a change looks like this:

1. The input line is broken up into these parts: `<file-path>:<line-number>:[<column-number>:]<line-content>`.
2. The substitution (e.g., the first and second [find and replace] arguments) are applied to the `<line-contents>`.
3. The result is written to the `<file-path>`.

This means editing standard input first, *and then also applying a find and replace via `rep`*, will also work (e.g., `rep bar baz < tmp`).

## Installation

`rep` is available via [`cargo`](https://github.com/rust-lang/cargo):

```
cargo install rep-grep
```

## Configuration

The default pager is `less`, the `REP_PAGER` environment variable can be used to override the pager (e.g., `export REP_PAGER=delta` in Bash).

## Help

`rep -h` (or `rep --help`, `--help` provides slightly longer explanations of some options) will list help for all the command-line flags.

## Acknowledgements

- `rep` was inspired by [`wgrep`](https://github.com/mhayashi1120/Emacs-wgrep) for Emacs, which allows editing `grep` results in an Emacs buffer and then writing those changes to the source files.
- Much of the functionality, and the overall structure of the source code, was borrowed [`sd`](https://github.com/chmln/sd). `rep` began as a fork of `sd`.
- The code for specifying a custom pager for `rep` was borrowed from [`delta`](https://github.com/dandavison/delta).
