# Rep Grep

`rep` is a command-line utility that takes [`grep`](https://en.wikipedia.org/wiki/Grep)-formatted lines via standard input, and performs a find-and-replace on them. By default, it outputs a [`diff`](https://en.wikipedia.org/wiki/Diff)-preview of the changes to standard output, and with a flag it can write the changes to the files in place.

[![Find & replace with `rep`](rep.gif)](https://www.youtube.com/embed/QIOKKTnC9-I)

## Example

Output a diff to standard output replacing `foo` with `bar`:

```
grep -n foo *` | rep foo bar
```

Add the `-w` flag to write the changes to the files in place:

```
grep -n foo *` | rep foo bar -w
```

The `-n` (`--line-number`) option is required so that `grep` outputs the line number for each match.

## Installation

`rep` is only available via [`cargo`](https://github.com/rust-lang/cargo):

```
cargo install rep-grep
```

## Help

`rep -h` (or `rep --help`) will list help for all the command-line flags.

## Acknowledgements

- `rep` was inspired by [`wgrep`](https://github.com/mhayashi1120/Emacs-wgrep) for Emacs, which allows editing `grep` results in an Emacs buffer and then writing those changes to the source files.
- Much of the functionality, and the overall structure of the source code, was borrowed [`sd`](https://github.com/chmln/sd). `rep` began as a fork of `sd`.
- The code for specifying a custom pager for `rep` was borrowed from [delta`](https://github.com/dandavison/delta).

