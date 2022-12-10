#[cfg(test)]
#[cfg(not(reap_cross_compile))] // Cross-compilation does not allow to spawn threads but `command.assert()` would do.
mod cli {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::io::prelude::*;

    fn reap() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking reap")
    }

    fn assert_file(path: &std::path::Path, content: &str) {
        assert_eq!(content, std::fs::read_to_string(path).unwrap());
    }

    fn create_soft_link<P: AsRef<std::path::Path>>(
        src: &P,
        dst: &P,
    ) -> Result<()> {
        #[cfg(target_family = "unix")]
        std::os::unix::fs::symlink(src, dst)?;
        #[cfg(target_family = "windows")]
        std::os::windows::fs::symlink_file(src, dst)?;

        Ok(())
    }

    #[test]
    fn stdin() -> Result<()> {
        reap().args(&["abc\\d+", ""])
            .write_stdin("abc123def")
            .assert()
            .success()
            .stdout("def");

        Ok(())
    }
}
