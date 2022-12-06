use crate::edit::Edit;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Patcher {
    edits: Vec<Edit>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid line number")]
    LineNumber,
}

impl Patcher {
    pub(crate) fn new(edits: Vec<Edit>) -> Self {
        Self { edits }
    }

    pub(crate) fn patch(&self, mut lines: Vec<String>) -> Result<String, Error> {
        let len = lines.len();
        for edit in &self.edits {
            if edit.number > len.try_into().unwrap() {
                return Err(Error::LineNumber);
            }
            let index = usize::try_from(edit.number).unwrap();
            lines[index] = edit.text.clone();
        }
        return Ok(lines.join("\n"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch() {
        let patcher = Patcher::new(vec![
            Edit {
                file: PathBuf::from("a"),
                number: 1,
                text: "foo".to_string(),
            },
            Edit {
                file: PathBuf::from("a"),
                number: 2,
                text: "bar".to_string(),
            },
        ]);
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = patcher.patch(lines);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a\nfoo\nbar");
    }
}
