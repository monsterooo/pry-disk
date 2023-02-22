use ::std::collections::{HashMap, VecDeque};
use ::std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileOrFolder {
  Folder(Folder),
  File(File),
}

impl FileOrFolder {
  pub fn size(&self) -> u128 {
    match self {
      FileOrFolder::Folder(folder) => folder.size,
      FileOrFolder::File(file) => file.size,
    }
  }
}

#[derive(Debug, Clone)]
pub struct File {
  pub name: OsString,
  pub size: u128,
}

#[derive(Debug, Clone)]
pub struct Folder {
  pub name: OsString,
  pub contents: HashMap<OsString, FileOrFolder>,
  pub size: u128,
  pub num_descendants: u64,
}

impl From<OsString> for Folder {
  fn from(name: OsString) -> Self {
    Folder {
      name,
      contents: HashMap::new(),
      size: 0,
      num_descendants: 0
    }
  }
}

impl Folder {
  pub fn new(path: &PathBuf) {
    
  }
}

