include!("src/cli.rs");

fn main() {
    use std::{env::var, fs};
    use clap::CommandFactory;
    use clap_complete::{generate_to, shells};

    // Rebuild when command-line options change
    println!("cargo:rerun-if-changed=src/cli.rs");

    // `generate_to` from `Clap` requires a mutable reference
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

    let man = clap_mangen::Man::new(cmd);
    let mut man_path = std::path::PathBuf::from(var("OUT_DIR").unwrap());
    man_path.push("rep.1");
    let mut out = std::fs::File::create(man_path).unwrap();
    man.render(&mut out).unwrap();
}
