use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::str::FromStr;

#[derive(Debug)]
pub struct Zip<'a> {
    path: &'a Path
}

impl<'a> Zip<'a> {
    pub fn from(path: &'a Path) -> Self {
        Zip {
            path
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        Zip::from(Path::new(s))
    }

    pub fn unzip(&self, directory: Option<String>) -> ExitStatus {
        let output_path = match directory {
            Some(val) => { val }
            None => ".".to_owned()
        };

        self._unzip(self.path.to_str().unwrap(), output_path.as_str())
    }

    pub fn _unzip(&self, source: &str, target: &str) -> ExitStatus {
        Command::new("unzip")
            .args(&["-n", source, "-d", target])
            .status().unwrap()
    }
}
