use std::env::current_dir;
use std::{env, fs};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
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
        writer.flush().expect("Flush Failed!");
    }

    fn load() -> Config {

        if !Path::new("config.json").exists() {
            let temp_conf = Config::new(env::current_dir().unwrap());
            temp_conf.save();

        }

        let file = File::open("config.json");
        let mut reader = BufReader::new(file.unwrap());
        serde_json::from_reader(&mut reader).expect("JSON Parse Failed!")

    }

    fn validate_storm_dir(&self) -> bool {
        let gamepath = self.storm_path.join("stormworks64.exe");
        Path::new(&gamepath).exists()
    }


}

fn craft_workshop_path(storm_path: PathBuf) -> PathBuf {
    let mut path = storm_path;
    path.pop();
    path.pop();
    path.push("workshop");
    path.push("content");
    path.push("573090");
    return if path.exists() {
        path
    } else {
        PathBuf::new()
    }

}

fn main() {
    let mut config = Config::load();

    if config.validate_storm_dir() {
        println!("Valid");
        config.storm_workshop_path = craft_workshop_path(config.storm_path);
        println!("{}", config.storm_workshop_path.display());
    }
    else {
        println!("Not valid");
    }
}
