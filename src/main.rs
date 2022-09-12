use clap::{CommandFactory, Parser};
use exitcode;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    fs,
    path::{self, Path},
};

use notify::{
    event::{DataChange::Content, ModifyKind::Data, ModifyKind::Name, RenameMode},
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

fn is_temp_file(file: &path::Path) -> bool {
    let _extension = OsStr::new("stmp");
    match file.extension() {
        Some(_extension) => true,
        None => false,
    }
}

fn watch(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let mut event_history: Vec<String> = Vec::new();
    let mut count = 0;
    for res in rx {
        match res {
            Ok(event) => {
                count += 1;
                debug!("{}-- -------------------\n{:?}", count, event);
                if event.kind == Modify(Name(RenameMode::Any)) {
                    info!("File changed: {:?}", event.paths[0]);

                    let source_str = event.paths[0].to_str().unwrap().to_string();
                    let source_path = event.paths[0].as_path();

                    // TODO Builder pattern?
                    if !is_temp_file(source_path) {
                        match event_history.last() {
                            Some(source_str) => {
                                debug!("File already in event_history: {:?}", &source_str);
                                let target_path = path::Path::new(source_path.file_name().unwrap());

                                debug!("copying {:?} to {:?}", source_path, target_path);

                                // TODO copy into backup_dir/$datetime
                                match fs::copy(source_path, target_path) {
                                    Ok(_) => info!("Copied {:?} to {:?}", source_path, target_path),
                                    Err(e) => error!("Error copying file: {}", e),
                                }
                                event_history.pop();
                            }
                            None => {
                                event_history.push(source_str);
                            }
                        }
                    }
                }
                if event.kind == Modify(Data(Content)) {
                    debug!(
                        "File modified: {:?} || {:?} || {:?}",
                        event.paths, event.attrs, event.kind
                    );
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
