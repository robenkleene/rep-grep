use std::path::PathBuf;
use regex::Regex;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
pub(crate) struct Edit {
    file: PathBuf,
    text: String,
    number: u32
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
  #[error("No line, number, or text matches")]
  Match,
}

impl Edit {
    fn new(file: PathBuf, text: String, number: u32) -> Edit {
        Edit { file, text, number }
    }

    pub(crate) fn parse<R> (
        reader: BufReader<R>
    ) -> Result<Vec::<Edit>, std::io::Error>
        where R: Read
    {
        let mut edits = Vec::<Edit>::new();
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };
            let line = match Self::edit_from_line(line) {
                Ok(line) => line,
                Err(_) => continue,
            };
            edits.push(line);
        }
        return Ok(edits);
    }

    fn edit_from_line(line: String) -> Result<Edit, Error> {
        let re = Regex::new("^([^:]+):([[:digit:]]+):(.*)$").unwrap();
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
        let mut text = match caps.get(3) {
            Some(text) => text.as_str().to_string(),
            None => return Err(Error::Match),
        };

        // Check for the optional column and discard it if present
        let text_re = Regex::new("^([[:digit:]]+):(.*)$").unwrap();
        let text_caps = text_re.captures(&line);
        if let Some(text_caps) = text_caps {
            text = match text_caps.get(2) {
                Some(text) => text.as_str().to_string(),
                None => text,
            };
        }

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
    fn edit_from_line_none() {
        let result = Edit::edit_from_line("aaa".to_string());
        assert!(matches!(result, Err(Error::Match)));
    }
}
