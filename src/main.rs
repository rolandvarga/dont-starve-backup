use clap::{CommandFactory, Parser};
use exitcode;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    fs,
    path::{self, Path, PathBuf},
};

use notify::{
    event::{ModifyKind::Name, RenameMode},
    Config,
    EventKind::Modify,
    RecommendedWatcher, RecursiveMode, Watcher,
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[derive(Parser)]
struct Cli {
    command: String,
}

#[derive(Debug, Deserialize)]
struct AppConf {
    save_dir: String,
    backup_dir: String,
}

struct EventFile {
    paths: Vec<PathBuf>,

    source_str: String,
    source_path: PathBuf,
    target_path: PathBuf,

    file_name: String,
    is_temp_file: bool,
}

impl EventFile {
    fn new(paths: &Vec<PathBuf>) -> EventFile {
        EventFile {
            paths: paths.clone(),

            source_str: String::new(),
            source_path: PathBuf::new(),
            target_path: PathBuf::new(),

            file_name: String::new(),
            is_temp_file: false,
        }
    }
    fn build_source(mut self) -> EventFile {
        self.source_str = self.paths[0].to_str().unwrap().to_string();
        self.source_path = self.paths[0].as_path().to_path_buf();
        self
    }
    fn build_target(mut self) -> EventFile {
        self.target_path = path::Path::new("backup").join(&self.source_path.file_name().unwrap());
        self
    }
    fn build_file_name_and_extension(mut self) -> EventFile {
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

fn read_file_at(path: &str) -> Result<String, std::io::Error> {
    let file = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    Ok(file)
}

fn parse_config_from(file: String) -> Result<AppConf, toml::de::Error> {
    let conf: AppConf = match toml::from_str(&file) {
        Ok(conf) => conf,
        Err(e) => return Err(e),
    };
    Ok(conf)
}

fn watch(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let mut event_history: Vec<String> = Vec::new();
    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == Modify(Name(RenameMode::Any)) {
                    let event_file = EventFile::new(&event.paths)
                        .build_source()
                        .build_target()
                        .build_file_name_and_extension();

                    if !event_file.is_temp_file {
                        let _tmp_source = &event_file.source_str;
                        match event_history.last() {
                            Some(_tmp_source) => {
                                debug!(
                                    "copying {:?} to {:?}",
                                    &event_file.source_path, &event_file.target_path
                                );

                                // TODO copy into backup_dir/$datetime
                                match fs::copy(&event_file.source_path, &event_file.target_path) {
                                    Ok(_) => info!(
                                        "Copied {:?} to {:?}",
                                        event_file.source_path, event_file.target_path
                                    ),
                                    Err(e) => error!("Error copying file: {}", e),
                                }
                                event_history.pop();
                            }
                            None => {
                                event_history.push(event_file.source_str);
                            }
                        }
                    }
                }
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn main() {
    pretty_env_logger::init(); // TODO use try_init() instead

    let file = read_file_at("config.toml").unwrap_or_else(|e| {
        eprintln!("Error reading config file: {}", e);
        std::process::exit(exitcode::OSFILE);
    });

    let cfg: AppConf = parse_config_from(file).unwrap_or_else(|e| {
        eprintln!("Error parsing config file: {}", e);
        std::process::exit(exitcode::CONFIG);
    });

    let args = Cli::parse();
    match args.command.as_str() {
        "backup" => {
            info!("starting in backup mode");
            // start a thread that keeps monitoring the save_dir
            // and copies any new files to the backup_dir when there's a change
            let path = path::Path::new(&cfg.save_dir);
            if let Err(e) = watch(path) {
                error!("Error watching save_dir: {}", e);
                std::process::exit(exitcode::OSERR);
            }
        }
        "restore" => {
            info!("starting in restore mode");
            // copy all files from the backup_dir to the save_dir
        }
        _ => {
            Cli::command().print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}
