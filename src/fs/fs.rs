extern crate serde;
extern crate serde_yaml;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Error;
use std::fs::{read_to_string, read_dir, write, DirEntry};
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_yaml::{from_str, to_string};

pub struct Filesystem {
    root: PathBuf
}

impl Filesystem {
    pub fn new(root: String) -> Self {
        Filesystem {
            root: PathBuf::from(root)
        }
    }

    pub fn get_path(&self, path: PathBuf) -> PathBuf {
        self.root.join(path)
    }

    pub fn get(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }

    /// Reads a single file via a `PathBuf` path.
    pub fn read_path(&self, path: PathBuf) -> Result<String, Error> {
        read_to_string(self.get_path(path))
    }

    /// Reads a single file via a `String` path.
    /// Errors if an IO error occurs, or if the file doesn't end with 'yml'.
    pub fn read(&self, path: &str) -> Result<String, String> {
        let p = self.get(path);

        if p.extension().unwrap() == "yml" {
            match read_to_string(p) {
                Ok(s) => Ok(s),
                Err(e) => Err(e.to_string())
            }
        } else {
            Err(format!("Invalid file type for {}", p.to_str().unwrap_or("(invalid file)")))
        }
    }

    /// Reads a directory and returns a `BTreeMap` of each filename and its contents.
    pub fn read_dir(&self, path: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
        let entries = read_dir(self.get(path))?;
        let mut result: BTreeMap<String, String> = BTreeMap::new();

        for entry in entries {
            let dir = entry?;

            let s = Self::get_file_name(&dir).ok_or("Invalid file path")?;
            let contents = self.read_path(dir.path())?;

            if s == "all" {
                result.clear();
                result.insert(s, contents);
                break;
            }
            
            result.insert(s, contents);
        }

        Ok(result)
    }

    /// Returns the filename of a directory as an `Option<String>`.
    /// Note that Rust only supports UTF-8 characters in Strings.
    pub fn get_file_name(d: &DirEntry) -> Option<String> {
        Some(String::from(
            d.path()
                .file_stem()
                .unwrap()
                .to_str()?
        ))
    }

    /// Parses some YAML data as a Deserialize-able struct.
    pub fn parse<T: DeserializeOwned>(data: String) -> Result<T, serde_yaml::Error> {
        from_str::<T>(data.as_str())
    }

    /// Goes through the result of a `Filesystem::read_dir` call and `parse`s each item.
    pub fn parse_map<T: DeserializeOwned>(data: BTreeMap<String, String>) -> Result<HashMap<String, T>, serde_yaml::Error> {
        if data.contains_key("all") {
            return Self::parse_all(data.get("all").unwrap().clone())
        }

        let mut result: HashMap<String, T> = HashMap::with_capacity(data.len());

        for d in data {
            let parsed = Self::parse(d.1)?;
            result.insert(d.0, parsed);
        }

        Ok(result)
    }

    /// Parses the 'all.yml' data style.
    pub fn parse_all<T: DeserializeOwned>(data: String) -> Result<HashMap<String, T>, serde_yaml::Error> {
        from_str::<HashMap<String, T>>(data.as_str())
    }

    pub fn encode<T: Serialize>(data: &T) -> Result<String, serde_yaml::Error> {
        to_string(data)
    }

    pub fn write<T: Serialize>(&self, data: &T, path: &String) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = Self::encode(data)?;
        write(self.get(path), encoded)?;

        Ok(())
    }
}