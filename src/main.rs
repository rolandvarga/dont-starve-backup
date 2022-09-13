mod config;
mod model;

use chrono::Duration;
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
#[clap(author = "rolandvarga", version, about = "A simple backup utility")]
struct Cli {
    #[clap(forbid_empty_values = true, validator = validate_arg )]
    /// command to execute. Options include 'monitor|restore'
    command: String,

    #[clap(forbid_empty_values = false, validator = validate_backup_dir )]
    /// path to the saved backup files
    backup_dir: String,
}

fn validate_arg(command: &str) -> Result<(), String> {
    match command {
        "monitor" | "restore" => Ok(()),
        _ => Err(String::from("Invalid command")),
    }
}

fn validate_backup_dir(backup_dir: &str) -> Result<(), String> {
    let path = path::Path::new(backup_dir);
    if path.exists() {
        Ok(())
    } else {
        Err(String::from("Invalid backup directory"))
    }
}

fn watch(cfg: AppConf) -> notify::Result<()> {
    let save_dir_path = path::Path::new(&cfg.save_dir);
    let backup_dir_path = path::Path::new(&cfg.backup_dir);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(save_dir_path.as_ref(), RecursiveMode::Recursive)?;

    let mut event_tracker = model::EventTracker::new();
    for res in rx {
        match res {
            Ok(event) => {
                if event_tracker.duration_since_last_backup()
                    >= Duration::seconds(cfg.cycle_interval)
                {
                    event_tracker.idle = true;
                }
                if event.kind == Modify(Name(RenameMode::Any)) {
                    let mut event_file = model::EventFile::new(&event.paths)
                        .build_source()
                        .build_file_name_and_extension();

                    if !event_file.is_temp_file {
                        let _tmp_source = &event_file.source_str;
                        match event_tracker.last() {
                            Some(_tmp_source) => {
                                debug!(
                                    "copying {:?} to {:?}",
                                    &event_file.source_path, &event_file.target_path
                                );
                                if event_tracker.is_idle() {
                                    event_tracker.start_cycle();
                                } else {
                                    event_tracker.update_last_backup();
                                }

                                event_file = event_file.build_target(
                                    &backup_dir_path.join(
                                        event_tracker
                                            .current_cycle
                                            .format("%Y%m%d_%H%M%S")
                                            .to_string(),
                                    ),
                                );
                                event_file.copy_to_target();
                                event_tracker.pop();
                            }
                            None => {
                                event_tracker.push(event_file.source_str);
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
    pretty_env_logger::init();

    let _cfg: AppConf = AppConf::new("config.toml");
    let _args = Cli::parse();

    match _args.command.as_str() {
        "monitor" => {
            info!("starting in backup mode");
            // start a thread that keeps monitoring the save_dir
            // and copies any new files to the backup_dir when there's a change
            if let Err(e) = watch(_cfg) {
                error!("Error watching save_dir: {}", e);
                std::process::exit(exitcode::OSERR);
            }
        }
        "restore" => {
            info!("starting in restore mode");
            // TODO copy all files from the backup_dir to the save_dir
        }
        _ => {
            Cli::command().print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}
