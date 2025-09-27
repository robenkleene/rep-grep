include!("src/cli.rs");

fn main() {
    use std::{env::var, fs, str::FromStr};
    use structopt::clap::Shell;

    let mut app = Options::clap();
    let out_dir = var("SHELL_COMPLETIONS_DIR").or(var("OUT_DIR")).unwrap();

    fs::create_dir_all(&out_dir).unwrap();

    Shell::variants().iter().for_each(|shell| {
        app.gen_completions("rep", Shell::from_str(shell).unwrap(), &out_dir);
    });

    create_man_page();
}

fn create_man_page() {
    use man::prelude::*;
    let page = Manual::new("rep")
        .flag(Flag::new().short("-w").long("--write").help(
            r#"Write the output to files directly (instead of outputting a patch)

If this flag is not present, the output will be the changes in patch format and the default pager will be `less`. The
environment variable REP_PAGER can be used to override the pager.
"#,
        ))
        .flag(
            Flag::new()
                .short("-d")
                .long("--delete-lines")
                .help("Delete matching lines."),
        )
        .flag(
            Flag::new()
                .short("-s")
                .long("--string-mode")
                .help("Treat expressions as non-regex strings."),
        )
        .flag(Flag::new().long("--no-color").help("Disable color."))
        .flag(
            Flag::new()
                .long("--color")
                .help("Enable color (the default if the output is a TTY)."),
        )
        .flag(
            Flag::new()
                .short("-V")
                .long("--version")
                .help("Prints version information."),
        )
        .flag(Flag::new().short("-f").long("--flags").help(
            r#"Treat expressions as non-regex strings.
/** Regex flags. May be combined (like `-f mc`).

c - case-sensitive
i - case-insensitive
m - multi-line matching
w - match full words only
"#,
        ))
        .arg(Arg::new("find"))
        .arg(Arg::new("replace_with"))
        .section(Section::new("STANDARD INPUT").text(
            r#"
Writing to files can also be accomplished by editing the contents of the grep output itself (and omitting the find and replace arguments).

This means for example a workflow like this will work:

1. grep -n foo * > tmp
2. sed -i '' s/foo/bar/g tmp
3. rep < tmp
4. rep -w < tmp
            "#))
        .section(Section::new("FLOW").text(
            r#"
The flow rep uses when making a change looks like this:

1. The input line is broken up into these parts: <file-path>:<line-number>:[<column-number>:]<line-content>
2. The the substitution (e.g., the first and second [find and replace] arguments) are applied to the <line-contents>
3. The result is written to the <file-path>

These means editing standard input first, and then applying a find and replace to the resulting grep-formatted lines, will work.
            "#))
        .render();
        .render();

    let mut man_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    man_path.push("rep.1");
    std::fs::write(man_path, page).expect("Error writing man page");
}
