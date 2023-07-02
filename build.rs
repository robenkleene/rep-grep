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
        .flag(
            Flag::new()
                .short("-w")
                .long("--write")
                .help(
            r#"Write the output to files directly (instead of outputting a patch)

If this flag is not present, and a patch is output, then the default pager is `less`. The
environment variable REP_PAGER can be used to override the pager.
"#,
        ))
        .flag(
            Flag::new()
                .short("-s")
                .long("--string-mode")
                .help("Treat expressions as non-regex strings."),
        )
        .flag(
            Flag::new()
                .long("--no-color")
                .help("Disable color."),
        )
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
        .render();

    let mut man_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    man_path.push("rep.1");
    std::fs::write(man_path, page).expect("Error writing man page");
}
