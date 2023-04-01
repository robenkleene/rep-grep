use std::ffi::OsString;
use std::io::StdinLock;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

enum Error {
    #[error("Could not parse pager command")]
    ParseError(String),
}

impl Output {
    pub(crate) fn handle(pager: Option<String>) -> Result<StdinLock<'static>, crate::output::Error> {
        let stdin = std::io::stdin();
        Ok(stdin.lock())
    }

    fn try_pager(
        pager: Option<String>,
        quit_if_one_screen: bool,
    ) -> Result<()> {

        let pager = pager.unwrap_or_else(|| String::from("less"));
        let pagerflags = match shell_words::split(&pager) {
            Ok(pagerflags) => pagerflags,
            Err(err) => return Err(Error::ParseError(pager)),
        };

        Ok(match pagerflags.split_first() {
            Some((pager_name, args)) => {
                let pager_path = PathBuf::from(pager_name);

                let is_less = pager_path.file_stem() == Some(&OsString::from("less"));

                let process = if is_less {
                    _make_process_from_less_path(
                        pager_path,
                        args,
                        quit_if_one_screen,
                    )
                } else {
                    _make_process_from_pager_path(pager_path, args)
                };
                if let Some(mut process) = process {
                    process
                        .stdin(Stdio::piped())
                        .spawn()
                        .map(OutputType::Pager)
                        .unwrap_or_else(|_| OutputType::stdout())
                } else {
                    OutputType::stdout()
                }
            }
            None => OutputType::stdout(),
        })
    }
}

fn _make_process_from_less_path(
    less_path: PathBuf,
    args: &[String],
    quit_if_one_screen: bool,
) -> Option<Command> {
    if let Ok(less_path) = grep_cli::resolve_binary(less_path) {
        let mut p = Command::new(&less_path);
        if args.is_empty() || replace_arguments_to_less {
            p.args(vec!["--RAW-CONTROL-CHARS"]);

            // Passing '--no-init' fixes a bug with '--quit-if-one-screen' in older
            // versions of 'less'. Unfortunately, it also breaks mouse-wheel support.
            //
            // See: http://www.greenwoodsoftware.com/less/news.530.html
            //
            // For newer versions (530 or 558 on Windows), we omit '--no-init' as it
            // is not needed anymore.
            match retrieve_less_version() {
                None => {
                    p.arg("--no-init");
                }
                Some(version) if (version < 530 || (cfg!(windows) && version < 558)) => {
                    p.arg("--no-init");
                }
                _ => {}
            }

            if quit_if_one_screen {
                p.arg("--quit-if-one-screen");
            }
        } else {
            p.args(args);
        }
        p.env("LESSCHARSET", "UTF-8");
        p.env("LESSANSIENDCHARS", "mK");
        Some(p)
    } else {
        None
    }
}

