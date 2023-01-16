# idm-rs

idm-rs is a crate to download files with Internet Download Manager.

## Requirements
[Internet Download Manager](https://www.internetdownloadmanager.com/) installed on your system.

Example usage:
```rust
use idm_rs::idman;
use std::env;

let mut downloader = idman::new();
downloader.set_download_file_url("https://www.tonec.com/download/idman317.exe");
downloader.set_download_file_name("Internet Download Manager.exe");
downloader.set_download_file_path(&env::current_dir().unwrap().as_path());
downloader.set_mode(idm_rs::Mode::Silent);
downloader.run();
```