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
        .arg(Arg::new("[FILES]"))
        .example(
            Example::new()
                .text("String-literal mode")
                .command(
                    "echo 'lots((([]))) of special chars' | rep -s '((([])))' \
                     ''",
                )
                .output("lots of special chars"),
        )
        .example(
            Example::new()
                .text("Regex use. Let's trim some trailing whitespace")
                .command("echo 'lorem ipsum 23   ' | rep '\\s+$' ''")
                .output("lorem ipsum 23"),
        )
        .example(
            Example::new()
                .text("Indexed capture groups")
                .command(r#"echo 'cargo +nightly watch' | rep '(\w+)\s+\+(\w+)\s+(\w+)' 'cmd: $1, channel: $2, subcmd: $3'"#)
                .output("cmd: cargo, channel: nightly, subcmd: watch")
        )
        .example(
            Example::new()
                .text("Named capture groups")
                .command(r#"echo "123.45" | rep '(?P<dollars>\d+)\.(?P<cents>\d+)' '$dollars dollars and $cents cents'"#)
                .output("123 dollars and 45 cents")
        )
        .example(
            Example::new()
                .text("Find & replace in file")
                .command(r#"rep 'window.fetch' 'fetch' http.js"#)
        )
        .example(
            Example::new()
                .text("Find & replace from STDIN an emit to STDOUT")
                .command(r#"rep 'window.fetch' 'fetch' < http.js"#)
        )
        .render();

    let mut man_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    man_path.push("rep.1");
    std::fs::write(man_path, page).expect("Error writing man page");
}
