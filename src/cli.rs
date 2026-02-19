use clap::Parser;

#[derive(Debug, Parser)]
// Specify `#[allow(dead_code)]` because the build script (`build.rs`) includes `cli.rs` but doesn't
// use these options which emits a warning.
#[allow(dead_code)]
#[clap(
    version,
    about = "Find and replace on grep output, or edit grep output to apply changes",
    long_about = "Find and replace on grep output, or edit grep output to apply changes.\n\n\
        rep takes grep-formatted lines (e.g., from ripgrep) and applies find and\n\
        replace on the matches.\n\n\
        EXAMPLES:\n  \
          grep -n foo * | rep foo bar       Show diff replacing foo with bar\n  \
          grep -n foo * | rep foo bar -w    Write the replacements to files\n\n\
        EDITING STANDARD INPUT:\n  \
          Changes can also be made by editing the grep output directly, without\n  \
          passing find and replace arguments:\n    \
            1. grep -n foo * > tmp           Save grep matches to a file\n    \
            2. Edit tmp in a text editor     Make changes to the matched lines\n    \
            3. rep < tmp                     Preview the diff\n    \
            4. rep -w < tmp                  Write the changes\n\n\
        INPUT FORMAT:\n  \
          Each input line has the format: <file>:<line>:[<column>:]<text>\n  \
          The -n (--line-number) grep flag is required for correct line numbers.\n\n\
        SPECIAL CHARACTERS:\n  \
          Use -- to separate options from arguments when the pattern starts\n  \
          with a hyphen (e.g., rep -- '--foo' '--bar').\n\n\
        PAGER:\n  \
          The default pager is less. Set the REP_PAGER environment variable to\n  \
          override (e.g., export REP_PAGER=delta).",
    disable_help_subcommand = true,
    next_line_help = true,
)]
pub(crate) struct Options {
    #[clap(short = 'w', long = "write")]
    /// Write the output to files directly (instead of outputting a patch). If this flag is not present, and a patch is output, then the default pager is 'less'. The environment variable REP_PAGER can be used to override the pager.
    pub write: bool,

    #[clap(short = 'd', long = "delete-lines")]
    /// Delete matching lines
    pub delete: bool,

    #[clap(short = 's', long = "string-mode")]
    /// Treat expressions as non-regex strings
    pub literal_mode: bool,

    #[clap(short = 'n')]
    /// Limit the number of replacements per line
    pub replacements: Option<usize>,

    #[clap(long = "color")]
    /// Enable color (the default if the output is a TTY)
    pub color: bool,

    #[clap(long = "no-color")]
    /// Disable color
    pub no_color: bool,

    #[clap(long = "stdout")]
    /// Force printing to standard output without using a pager
    pub stdout: bool,

    #[clap(short = 'f', long = "flags")]
    /// Regex flags, may be combined (like '-f mc'). 'c': case-sensitive, 'e': disable multi-line matching, 'i': case-insensitive, 'm': multi-line matching, 's': make '.' match newlines, 'w': match full words only.
    pub flags: Option<String>,

    /// The regexp or string (if -s) to search for.
    pub find: Option<String>,

    /// What to replace each match with. Unless in string mode, you may
    /// use captured values like $1, $2, etc.
    pub replace_with: Option<String>,
}
