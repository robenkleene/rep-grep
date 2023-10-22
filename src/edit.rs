use std::path::PathBuf;
use std::io::StdinLock;
use regex::Regex;
use std::io::prelude::*;
use indexmap::IndexMap;

#[derive(Debug)]
pub(crate) struct Edit {
    pub(crate) file: PathBuf,
    pub(crate) text: String,
    pub(crate) line_number: u32
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
  #[error("No file, number, and text matches")]
  Match,
}

impl Edit {
    pub(crate) fn new(file: PathBuf, text: String, line_number: u32) -> Edit {
        Edit { file, text, line_number }
    }

    pub(crate) fn parse (
        reader: StdinLock<'_>
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
        return Ok(path_to_edits);
    }

    fn edit_from_line(line: String) -> Result<Edit, Error> {
        // The last `([[:digit:]]+:)?` capture group is for the optional column, which we only
        // discard
        let re = Regex::new("^([^:]+):([[:digit:]]+):([[:digit:]]+:)?(.*)$").unwrap();
        let caps = re.captures(&line);
        let caps = match caps {
            Some(caps) => caps,
            None => return Err(Error::Match),
        };
        let file = match caps.get(1) {
            Some(file) => PathBuf::from(file.as_str()),
            None => return Err(Error::Match),
        };
        let number = match caps.get(2) {
            Some(number) => number.as_str(),
            None => return Err(Error::Match),
        };
        let number = match number.parse::<u32>() {
            Ok(number) => number,
            Err(_) => return Err(Error::Match),
        };
        // If `caps.len() > 4`, then the optional column was present
        let index = if caps.len() < 5 { 3 } else { 4 };
        let text = match caps.get(index) {
            Some(text) => text.as_str().to_string(),
            None => return Err(Error::Match),
        };

        return Ok(Edit::new(
            file,
            text,
            number,
        ))
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
