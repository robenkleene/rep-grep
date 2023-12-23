# Rep Grep

`rep` is a command-line utility that takes [`grep`](https://en.wikipedia.org/wiki/Grep)-formatted lines on standard in, and can perform a find-and-replace on them, preview the changes as a [`diff`](https://en.wikipedia.org/wiki/Diff), and then write the changes to the source files.

[![Find & replace with `rep`](rep.gif)](https://www.youtube.com/embed/QIOKKTnC9-I)

## Example

Output a diff to standard output of a replacement of `foo` with `bar`:

```
grep -n foo *` | rep foo bar
```

Write the changes to the files with matches:

```
grep -n foo *` | rep foo bar -w
```

The `-n` (`--line-number`) option is necessary so `grep` outputs the line number of each match.

## Help

`grep --help` (or `grep -h`) will list help for all the command-line flags.

## Acknowledgements

- The idea for `rep` was inspired by [`wgrep`](https://github.com/mhayashi1120/Emacs-wgrep) for Emacs, which allows editing of a `grep` results in a buffer, and writing those changes to the source files.
- The structure of the source code, and much of the functionality, was borrowed from [`sd`](https://github.com/chmln/sd), `rep` began as a fork of `sd`.
- The code for specifying a custom pager for both `rep` and `ren` came from the source code for [delta`](https://github.com/dandavison/delta).

