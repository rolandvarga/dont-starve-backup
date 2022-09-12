use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub struct EventFile {
    paths: Vec<PathBuf>,

    pub source_str: String,
    pub source_path: PathBuf,
    pub target_path: PathBuf,

    pub file_name: String,
    pub is_temp_file: bool,
}

impl EventFile {
    pub fn new(paths: &Vec<PathBuf>) -> EventFile {
        EventFile {
            paths: paths.clone(),

            source_str: String::new(),
            source_path: PathBuf::new(),
            target_path: PathBuf::new(),

            file_name: String::new(),
            is_temp_file: false,
        }
    }
    pub fn build_source(mut self) -> EventFile {
        self.source_str = self.paths[0].to_str().unwrap().to_string();
        self.source_path = self.paths[0].as_path().to_path_buf();
        self
    }
    pub fn build_target(mut self, prefix: &Path) -> EventFile {
        // TODO should handle backupdir/$datetime
        self.target_path = prefix.join(&self.source_path.file_name().unwrap());
        self
    }
    pub fn build_file_name_and_extension(mut self) -> EventFile {
        self.file_name = self
            .source_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let _extension: &OsStr = OsStr::new("stmp");
        match self.source_path.extension() {
            Some(_extension) => self.is_temp_file = true,
            None => self.is_temp_file = false,
        }
        self
    }
}
