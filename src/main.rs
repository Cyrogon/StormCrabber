use std::env::current_dir;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Config {
    storm_path: PathBuf,
    storm_workshop_path: PathBuf,
    backup_path: PathBuf,
    output_path: PathBuf,
}

impl Config {
    fn new(current_dir: PathBuf) -> Config {
        Config {
            storm_path: PathBuf::new(),
            storm_workshop_path: PathBuf::new(),
            backup_path: current_dir.join("backup"),
            output_path: current_dir.join("output"),
        }
    }

    fn save(self) {
        let file = File::create("config.json");
        let mut writer = BufWriter::new(file.unwrap());
        serde_json::to_writer_pretty(&mut writer, &self).expect("File Write Failed!");
    }
}

fn main() {
    let cwd = current_dir();

    let mut conf = Config::new(cwd.unwrap_or(PathBuf::new()));
    conf.save();
}
