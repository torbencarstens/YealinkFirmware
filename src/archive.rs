use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

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

    #[allow(dead_code)]
    pub fn from_str(s: &'a str) -> Self {
        Zip::from(Path::new(s))
    }

    pub fn unzip(&self, directory: Option<String>, remove: Option<bool>) -> ExitStatus {
        let output_path = match directory {
            Some(val) => { val }
            None => ".".to_owned()
        };

        self._unzip(self.path.to_str().unwrap(), output_path.as_str(), remove.unwrap_or(false))
    }

    fn _unzip(&self, source: &str, target: &str, remove: bool) -> ExitStatus {
        let status = Command::new("unzip")
            .args(&["-n", source, "-d", target])
            .status().unwrap();

        if remove {
            let _ = self.remove();
        }

        status
    }

    pub fn remove(&self) -> io::Result<()> {
        match fs::remove_file(self.path) {
            Ok(val) => {
                println!("Successfully deleted .zip file.");
                Ok(val)
            }
            Err(e) => {
                println!("Failed to delete .zip file: {:?}", e);
                Ok(())
            }
        }
    }
}
