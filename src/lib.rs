//! # idm-rs
//!
//! `idm-rs` is a crate to download files with Internet Download Manager.
//! Example usage:
//! ```rust
//! use idm_rs::idman;
//! use std::env;
//! 

//!let mut downloader = idman::new();
//!downloader.set_download_file_url("https://www.tonec.com/download/idman317.exe");
//!downloader.set_download_file_name("Internet Download Manager.exe");
//!downloader.set_download_file_path(&env::current_dir().unwrap().as_path());
//!downloader.set_mode(idm_rs::Mode::Silent);
//!downloader.run();
//! ```

use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

pub enum Mode {
    Default,
    /// Turns on the silent mode when IDM doesn't ask any questions.
    Silent,
}

#[allow(non_camel_case_types)]
pub struct idman {
    idm_path: PathBuf,
    mode: Mode,
    download_file_url: String,
    download_file_path: Option<PathBuf>,
    download_file_name: Option<String>,
}

impl idman {
    /// Create a new builder.
    pub fn new() -> Self {
        idman {
            idm_path: PathBuf::from(
                "C:\\Program Files (x86)\\Internet Download Manager\\IDMan.exe",
            ),
            mode: Mode::Default,
            download_file_url: String::new(),
            download_file_path: None,
            download_file_name: None,
        }
    }

    /// Set the path to the IDMan executable.
    /// Give this func a full path, with the .exe at the end.
    /// This function is optional, the default IDMan.exe path is the default install location.
    pub fn set_idm_path(&mut self, idm_path: &Path) {
        self.idm_path = idm_path.to_path_buf();
    }

    /// Set the download mode.
    /// This function is optional, the default mode is default.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    /// Set the url to the file.
    pub fn set_download_file_url(&mut self, url: &str) {
        self.download_file_url = url.to_string();
    }

    /// Set the path where the file will be downloaded.
    /// This function is optional, the default download path is handled by IDM.
    pub fn set_download_file_path(&mut self, file_path: &Path) {
        self.download_file_path = Some(file_path.to_path_buf());
    }

    /// Set the file name, don't forget to add extension at the end.
    /// If not set, IDM will detect it automatically.
    /// This function is optional, the default file name is handled by IDM.
    pub fn set_download_file_name(&mut self, file_name: &str) {
        self.download_file_name = Some(file_name.to_string());
    }

    fn process_args(&self) -> Vec<&str> {
        let mut args: Vec<&str> = Vec::new();

        match &self.mode {
            Mode::Default => {}
            Mode::Silent => args.push("/n"),
        }

        args.push("/d");
        args.push(&self.download_file_url);

        match &self.download_file_path {
            Some(x) => {
                args.push("/p");
                args.push(x.to_str().unwrap());
            }
            None => {}
        }

        match &self.download_file_name {
            Some(x) => {
                args.push("/f");
                args.push(x);
            }
            None => {}
        }

        args
    }

    /// Executes the settings and download the file.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        Command::new(self.idm_path.to_str().unwrap())
            .args(self.process_args())
            .output()?;
        Ok(())
    }
}
