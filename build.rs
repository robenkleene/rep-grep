include!("src/cli.rs");

fn main() {
    use std::{env::var, fs};
    use clap::CommandFactory;
    use clap_complete::{generate_to, shells};

    // Rebuild when command-line options change
    println!("cargo:rerun-if-changed=src/cli.rs");

    let mut cmd = Options::command();
    // Allowing scripts to override the `OUT_DIR` with an `SHELL_COMPLETIONS_DIR` is a convention
    // for Rust packages
    let out_dir = var("SHELL_COMPLETIONS_DIR")
        .or(var("OUT_DIR"))
        .expect("Either SHELL_COMPLETIONS_DIR or OUT_DIR environment variable must be set");

    fs::create_dir_all(&out_dir).expect("Failed to create completions output directory");
    let out_path = std::path::Path::new(&out_dir);

    generate_to(shells::Bash, &mut cmd, "rep", out_path).expect("Failed to generate Bash completion");
    generate_to(shells::Zsh, &mut cmd, "rep", out_path).expect("Failed to generate Zsh completion");
    generate_to(shells::Fish, &mut cmd, "rep", out_path).expect("Failed to generate Fish completion");
    generate_to(shells::PowerShell, &mut cmd, "rep", out_path).expect("Failed to generate Powershell completion");
    generate_to(shells::Elvish, &mut cmd, "rep", out_path).expect("Failed to generate Elvish completion");

    create_man_page(&mut cmd);
}

fn create_man_page(cmd: &mut clap::Command) {
    use man::prelude::*;

    // Build a Manual from the clap Command so help text stays in sync
    let name = cmd.get_name().to_string();
    let mut manual = Manual::new(&name);

    // Store owned strings so we can pass stable &str references into the man API
    let mut owned: Vec<String> = Vec::new();

    // Add a brief synopsis / about text if available
    if let Some(about) = cmd.get_about() {
        owned.push(about.to_string());
        manual = manual.about(owned.last().unwrap().as_str());
    } else if let Some(long_about) = cmd.get_long_about() {
        owned.push(long_about.to_string());
        manual = manual.about(owned.last().unwrap().as_str());
    }

    // Iterate arguments and flags from the clap Command and add them to the man page
    for arg in cmd.get_arguments() {
        // Skip internal clap args (like help) if they have no visible name
        let id = arg.get_id().to_string();

        if arg.get_index().is_some() {
            // Positional argument
            manual = manual.arg(Arg::new(&id));
            continue;
        }

        // Options / flags
        let mut flag = Flag::new();

        if let Some(s) = arg.get_short() {
            owned.push(format!("-{}", s));
            flag = flag.short(owned.last().unwrap().as_str());
        }
        if let Some(l) = arg.get_long() {
            owned.push(format!("--{}", l));
            flag = flag.long(owned.last().unwrap().as_str());
        }

        if let Some(help) = arg.get_help() {
            owned.push(help.to_string());
            flag = flag.help(owned.last().unwrap().as_str());
        } else if let Some(long_help) = arg.get_long_help() {
            owned.push(long_help.to_string());
            flag = flag.help(owned.last().unwrap().as_str());
        }

        manual = manual.flag(flag);
    }

    let page = manual.render();

    let mut man_path = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    man_path.push(format!("{}.1", name));
    std::fs::write(man_path, page).expect("Error writing man page");
}
