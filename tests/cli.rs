#[cfg(test)]
#[cfg(not(reap_cross_compile))] // Cross-compilation does not allow to spawn threads but `command.assert()` would do.

mod cli {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::fs;

    fn reap() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking reap")
    }

    #[test]
    fn patch_preview_markdown() -> Result<()> {
        let input = fs::read_to_string("tests/data/markup-grep.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/markdown-markup.patch").expect("Error reading input");
        reap()
            .current_dir("tests/data/markdown")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    fn patch_preview_simple() -> Result<()> {
        let input = fs::read_to_string("tests/data/markup-grep.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/markdown-markup.patch").expect("Error reading input");
        reap()
            .current_dir("tests/data/simple")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }
}
