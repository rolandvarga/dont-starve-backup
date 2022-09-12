mod config;
mod model;

use clap::{CommandFactory, Parser};
use config::AppConf;
use exitcode;
use std::{fs, path};

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

fn watch(cfg: AppConf) -> notify::Result<()> {
    let save_dir_path = path::Path::new(&cfg.save_dir);
    let backup_dir_path = path::Path::new(&cfg.backup_dir);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(save_dir_path.as_ref(), RecursiveMode::Recursive)?;

    let mut event_history: Vec<String> = Vec::new();
    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == Modify(Name(RenameMode::Any)) {
                    let event_file = model::EventFile::new(&event.paths)
                        .build_source()
                        .build_target(backup_dir_path)
                        .build_file_name_and_extension();

                    if !event_file.is_temp_file {
                        let _tmp_source = &event_file.source_str;
                        match event_history.last() {
                            Some(_tmp_source) => {
                                debug!(
                                    "copying {:?} to {:?}",
                                    &event_file.source_path, &event_file.target_path
                                );

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

    let cfg: AppConf = AppConf::new("config.toml");
    let args = Cli::parse();

    match args.command.as_str() {
        "backup" => {
            info!("starting in backup mode");
            // start a thread that keeps monitoring the save_dir
            // and copies any new files to the backup_dir when there's a change
            if let Err(e) = watch(cfg) {
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
