use std::io::StdinLock;

pub(crate) struct Output { }

impl Output {
    pub(crate) fn handle() -> StdinLock<'static> {
        let stdin = std::io::stdin();
        stdin.lock()
    }
}
