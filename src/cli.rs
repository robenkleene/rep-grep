use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
// Specify `#[allow(dead_code)]` because the build script (`build.rs`) includes `cli.rs` but
// doesn't use these options which emits a warning.
#[allow(dead_code)]
#[structopt(
    setting(AppSettings::ColoredHelp),
    setting(AppSettings::NextLineHelp),
    setting(AppSettings::UnifiedHelpMessage)
)]
pub(crate) struct Options {
    #[structopt(short = "w", long = "write")]
    ///
    /** Write the output to files directly (instead of outputting a patch)

    If this flag is not present, and a patch is output, then the default pager is `less`. The
    environment variable REP_PAGER can be used to override the pager.
        */
    pub write: bool,

    #[structopt(short = "d", long = "delete-lines")]
    /// Delete matching lines
    pub delete: bool,

    #[structopt(short = "s", long = "string-mode")]
    /// Treat expressions as non-regex strings
    pub literal_mode: bool,

    #[structopt(short = "n")]
    /// Limit the number of replacements per line
    pub replacements: Option<usize>,

    #[structopt(long = "color")]
    /// Enable color (the default if the output is a TTY)
    pub color: bool,

    #[structopt(long = "no-color")]
    /// Disable color
    pub no_color: bool,

    #[structopt(long = "stdout")]
    /// Force printing to standard output without using a pager
    pub stdout: bool,

    #[structopt(short = "f", long = "flags", verbatim_doc_comment)]
    #[rustfmt::skip]
    /** Regex flags. May be combined (like `-f mc`)

c - case-sensitive
e - disable multi-line matching
i - case-insensitive
m - multi-line matching
s - make `.` match newlines
w - match full words only
{n}{n}
    */
    pub flags: Option<String>,

    /// The regexp or string (if -s) to search for.
    pub find: Option<String>,

    /// What to replace each match with. Unless in string mode, you may
    /// use captured values like $1, $2, etc.
    pub replace_with: Option<String>,
}
