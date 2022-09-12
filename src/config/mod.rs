use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConf {
    pub save_dir: String,
    pub backup_dir: String,
}

impl AppConf {
    pub fn new(path: &str) -> AppConf {
        let file = read_file_at(path).unwrap_or_else(|e| {
            eprintln!("Error reading config file: {}", e);
            std::process::exit(exitcode::OSFILE);
        });

        let cfg: AppConf = parse_config_from(file).unwrap_or_else(|e| {
            eprintln!("Error parsing config file: {}", e);
            std::process::exit(exitcode::CONFIG);
        });
        cfg
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
