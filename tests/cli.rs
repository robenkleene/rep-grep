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
        let input = fs::read_to_string("tests/data/markdown/markdown-to-markup-grep.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/markdown/markdown-markup.patch").expect("Error reading input");
        reap()
            .current_dir("tests/data/markdown")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn patch_preview_simple() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/grep.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/simple/patch.patch").expect("Error reading input");
        reap()
            .current_dir("tests/data/simple")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn patch_preview_files_stdin() -> Result<()> {
        let input = fs::read_to_string("tests/data/files/changes-to-altered-grep.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/files/patch.patch").expect("Error reading input");
        reap()
            .current_dir("tests/data/files")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn patch_preview_files_args() -> Result<()> {
        let input = fs::read_to_string("tests/data/files/grep.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/files/patch.patch").expect("Error reading input");
        reap()
            .current_dir("tests/data/files")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }
}
