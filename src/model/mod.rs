use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    path,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Duration, Local};

pub struct EventTracker {
    history: Vec<String>,

    pub idle: bool,
    pub current_cycle: DateTime<Local>, // timestamp of current backup cycle
    pub last_backup: DateTime<Local>,   // actual timestamp of last backup
}

impl EventTracker {
    pub fn new() -> EventTracker {
        let now = Local::now();
        EventTracker {
            history: Vec::new(),
            idle: true,
            current_cycle: now,
            last_backup: now,
        }
    }
    pub fn push(&mut self, event: String) {
        self.history.push(event);
    }
    pub fn pop(&mut self) {
        self.history.pop();
    }
    pub fn last(&self) -> Option<&String> {
        self.history.last()
    }
    pub fn is_idle(&self) -> bool {
        self.idle == true
    }
    pub fn start_cycle(&mut self) {
        self.idle = false;
        self.update_current_cycle();
        self.update_last_backup();
    }
    fn update_current_cycle(&mut self) {
        self.current_cycle = Local::now();
    }
    pub fn update_last_backup(&mut self) {
        self.last_backup = Local::now();
    }
    pub fn duration_since_last_backup(&self) -> Duration {
        Local::now() - self.last_backup
    }
}

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
    pub fn copy_to_target(self) {
        let parent_dir = &self.target_path.parent().unwrap();
        fs::create_dir_all(path::Path::new(parent_dir)).unwrap_or_else(|e| {
            error!("Error creating directory '{:?}': {}", parent_dir, e);
            std::process::exit(exitcode::OSERR);
        });
        match fs::copy(&self.source_path, &self.target_path) {
            Ok(_) => info!("Copied {:?} to {:?}", &self.source_path, &self.target_path),
            Err(e) => error!("Error copying file: {}", e),
        }
    }
}

pub struct RestoreFile {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
}

impl RestoreFile {
    pub fn new(save_dir: &String, entry: &DirEntry) -> RestoreFile {
        let source_path = entry.path();
        let target_path = path::Path::new(&save_dir).join(source_path.file_name().unwrap());
        RestoreFile {
            source_path,
            target_path,
        }
    }
    pub fn copy(self) {
        fs::copy(&self.source_path, &self.target_path).unwrap_or_else(|e| {
            error!(
                "Error copying {:?} to {:?}: {}",
                self.source_path, self.target_path, e
            );
            std::process::exit(exitcode::IOERR);
        });
        info!("Copied {:?} to {:?}", self.source_path, self.target_path);
    }
}
