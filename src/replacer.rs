use crate::{utils, Result};
use regex::bytes::Regex;

pub(crate) struct Replacer {
    regex: Regex,
    replace_with: Vec<u8>,
    is_literal: bool,
    replacements: usize,
}

impl Replacer {
    pub(crate) fn new(
        look_for: String,
        replace_with: String,
        is_literal: bool,
        flags: Option<String>,
        replacements: Option<usize>,
    ) -> Result<Self> {
        let (look_for, replace_with) = if is_literal {
            (regex::escape(&look_for), replace_with.into_bytes())
        } else {
            (
                look_for,
                utils::unescape(&replace_with)
                    .unwrap_or_else(|| replace_with)
                    .into_bytes(),
            )
        };

        let mut regex = regex::bytes::RegexBuilder::new(&look_for);
        regex.multi_line(true);

        if let Some(flags) = flags {
            flags.chars().for_each(|c| {
                #[rustfmt::skip]
                match c {
                    'c' => { regex.case_insensitive(false); },
                    'i' => { regex.case_insensitive(true); },
                    'm' => {},
                    'e' => { regex.multi_line(false); },
                    's' => {
                        if !flags.contains("m") {
                            regex.multi_line(false);
                        }
                        regex.dot_matches_new_line(true);
                    },
                    'w' => {
                        regex = regex::bytes::RegexBuilder::new(&format!(
                            "\\b{}\\b",
                            look_for
                        ));
                    },
                    _ => {},
                };
            });
        };

        Ok(Self {
            regex: regex.build()?,
            replace_with,
            is_literal,
            replacements: replacements.unwrap_or(0),
        })
    }

    pub(crate) fn replace<'a>(
        &'a self,
        content: &'a [u8],
    ) -> std::borrow::Cow<'a, [u8]> {
        if self.is_literal {
            self.regex.replacen(
                &content,
                self.replacements,
                regex::bytes::NoExpand(&self.replace_with),
            )
        } else {
            self.regex.replacen(
                &content,
                self.replacements,
                &*self.replace_with,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn replace<'a>(
        look_for: impl Into<String>,
        replace_with: impl Into<String>,
        literal: bool,
        flags: Option<&'static str>,
        src: &'static str,
        target: &'static str,
    ) {
        let replacer = Replacer::new(
            look_for.into(),
            replace_with.into(),
            literal,
            flags.map(ToOwned::to_owned),
            None,
        )
        .unwrap();
        assert_eq!(
            std::str::from_utf8(&replacer.replace(src.as_bytes())),
            Ok(target)
        );
    }

    #[test]
    fn default_global() {
        replace("a", "b", false, None, "aaa", "bbb");
    }

    #[test]
    fn escaped_char_preservation() {
        replace("a", "b", false, None, "a\\n", "b\\n");
    }

    #[test]
    fn case_sensitive_default() {
        replace("abc", "x", false, None, "abcABC", "xABC");
        replace("abc", "x", true, None, "abcABC", "xABC");
    }

    #[test]
    fn sanity_check_literal_replacements() {
        replace("((special[]))", "x", true, None, "((special[]))y", "xy");
    }

    #[test]
    fn unescape_regex_replacements() {
        replace("test", r"\n", false, None, "testtest", "\n\n");
    }

    #[test]
    fn no_unescape_literal_replacements() {
        replace("test", r"\n", true, None, "testtest", r"\n\n");
    }

    #[test]
    fn full_word_replace() {
        replace("abc", "def", false, Some("w"), "abcd abc", "abcd def");
    }
}
