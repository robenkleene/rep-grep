use std::io::StdinLock;
use shell_words;

pub(crate) struct Output { }

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not parse pager command")]
    ParseError(String),
}

impl Output {
    pub(crate) fn handle(pager: Option<String>) -> Result<StdinLock<'static>, crate::output::Error> {
        let pager = pager.unwrap_or_else(|| String::from("less"));
        let pagerflags =
            shell_words::split(&pager).ok_or_else(|| Error::ParseError(pager))?;

        let stdin = std::io::stdin();
        Ok(stdin.lock())
    }
}
