use crate::edit::Edit;
use crate::replacer::Replacer;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str;

pub(crate) struct Patcher<'a> {
    edits: Vec<Edit>,
    replacer: Option<&'a Replacer>
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid line number")]
    LineNumber,
}

impl<'a> Patcher<'a> {
    pub(crate) fn new(edits: Vec<Edit>, replacer: Option<&'a Replacer>) -> Self {
        Self { edits, replacer }
    }

    pub(crate) fn patch(&self, mut lines: Vec<String>) -> Result<String, Error> {
        for edit in &self.edits {
            // Subtract `1` from the line number because line numbers start from `1` and array
            // indices start from `0`
            let index = usize::try_from(edit.number).unwrap() - 1;
            if index >= lines.len().try_into().unwrap() {
                return Err(Error::LineNumber);
            }
            if let Some(replacer) = &self.replacer {
                let replaced = &replacer.replace(edit.text.as_bytes());
                let result = str::from_utf8(replaced);
                let text = match result {
                    Ok(result) => result,
                    Err(err) => panic!("Error replacing: {}", err), // FIXME:
                };
                lines[index] = text.to_string();
            } else {
                lines[index] = edit.text.clone();
            }
        }
        return Ok(lines.join("\n"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn patch_bad_number() {
        let patcher = Patcher::new(vec![
            Edit {
                file: PathBuf::from("f"),
                number: 1,
                text: "foo".to_string(),
            },
            Edit {
                file: PathBuf::from("f"),
                number: 3,
                text: "bar".to_string(),
            },
        ], None);
        let lines = vec!["a".to_string(), "b".to_string()];
        let result = patcher.patch(lines);
        assert!(matches!(result, Err(Error::LineNumber)));
    }

    #[test]
    fn patch() {
        let patcher = Patcher::new(vec![
            Edit {
                file: PathBuf::from("f"),
                number: 2,
                text: "foo".to_string(),
            },
            Edit {
                file: PathBuf::from("f"),
                number: 3,
                text: "bar".to_string(),
            },
        ], None);
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = patcher.patch(lines);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a\nfoo\nbar");
    }
}
