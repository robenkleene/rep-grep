#[cfg(test)]
#[cfg(not(reap_cross_compile))] // Cross-compilation does not allow to spawn threads but `command.assert()` would do.
mod cli {
    use anyhow::Result;
    use assert_cmd::Command;

    fn reap() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking reap")
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
