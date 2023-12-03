use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum Data {
    Sample,
    Full,
}

impl Data {
    fn label(&self) -> &str {
        match self {
            Self::Sample => "sample",
            Self::Full => "full"
        }
    }

    fn path(&self, year: u16, day: u8) -> PathBuf {
        let binding = env::var("CARGO_MANIFEST_DIR").unwrap();
        let root_dir = Path::new(&binding).parent().unwrap().to_str().unwrap();
        let label = self.label();
        let input_path = format!("{root_dir}/data/{year}/{day}-{label}");
        Path::new(&input_path.clone()).to_owned()
    }

    pub fn load_to_string(&self, year: u16, day: u8, session_cookie: &str) -> String {
        match fs::read_to_string(self.path(year, day)) {
            Ok(t) => t,
            Err(_) => {
                println!("Downloading missing data");
                self.download(year, day, session_cookie);
                fs::read_to_string(self.path(year, day)).unwrap()
            }
        }
    }

    pub fn download(&self, year: u16, day: u8, session_cookie: &str) {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let body = ureq::get(&url)
            .set("Cookie", &format!("session={}", session_cookie))
            .call().unwrap()
            .into_string().unwrap();

        // Specify the file path to save the input data
        let file_path = self.path(year, day);


        // Create all directories in the path if they are missing
        if let Some(parent_dir) = Path::new(&file_path).parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).expect("Failed to create directories");
            }
        }

        // Create a new file or overwrite existing file and write the input data to it
        let mut file = File::create(&file_path).expect("Failed to create file");
        file.write_all(body.as_bytes()).expect("Failed to write to file");

        println!("Data saved to {:?}", file_path.to_str().unwrap());
    }
}
