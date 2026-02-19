use indexmap::IndexMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::StdinLock;
use std::path::PathBuf;
use std::sync::LazyLock;

static EDIT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^([^:]+):([[:digit:]]+):([[:digit:]]+:)?(.*)$").unwrap()
});

#[derive(Debug)]
pub(crate) struct Edit {
    pub(crate) file: PathBuf,
    pub(crate) text: String,
    pub(crate) line_number: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No file, number, and text matches")]
    Match,
}

impl Edit {
    pub(crate) fn new(file: PathBuf, text: String, line_number: u32) -> Edit {
        Edit {
            file,
            text,
            line_number,
        }
    }

    pub(crate) fn parse(
        reader: StdinLock<'_>,
    ) -> Result<IndexMap<PathBuf, Vec<Edit>>, std::io::Error> {
        let mut path_to_edits = IndexMap::new();
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };
            let line = match Self::edit_from_line(line) {
                Ok(line) => line,
                Err(_) => continue,
            };
            let key = &line.file;
            if !path_to_edits.contains_key(key) {
                path_to_edits.insert(line.file.clone(), Vec::new());
            }
            path_to_edits.get_mut(key).unwrap().push(line);
        }
        Ok(path_to_edits)
    }

    fn edit_from_line(line: String) -> Result<Edit, Error> {
        let caps = EDIT_RE.captures(&line).ok_or(Error::Match)?;
        let file = PathBuf::from(caps.get(1).ok_or(Error::Match)?.as_str());
        let number = caps
            .get(2)
            .ok_or(Error::Match)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| Error::Match)?;
        // Group 4 is always the text; group 3 is the optional column (discarded)
        let text = caps.get(4).ok_or(Error::Match)?.as_str().to_string();

        Ok(Edit::new(file, text, number))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_from_line_no_file() {
        let result = Edit::edit_from_line("aaa".to_string());
        assert!(matches!(result, Err(Error::Match)));
    }

    #[test]
    fn edit_from_line_no_number() {
        let result = Edit::edit_from_line("aaa.txt:bbb:ccc".to_string());
        assert!(matches!(result, Err(Error::Match)));
    }

    #[test]
    fn edit_from_line_no_text() {
        let result = Edit::edit_from_line("aaa.txt:1".to_string());
        assert!(matches!(result, Err(Error::Match)));
    }

    #[test]
    fn edit_from_line() {
        let result = Edit::edit_from_line("aaa.txt:1:text".to_string());
        assert!(matches!(result, Ok(_)));
        let edit = match result {
            Ok(result) => result,
            Err(_) => panic!("Error getting edit from line"),
        };
        assert_eq!(edit.file, PathBuf::from("aaa.txt"));
        assert_eq!(edit.line_number, 1);
        assert_eq!(edit.text, "text");
    }
}
