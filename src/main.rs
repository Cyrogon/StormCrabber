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

    fn save(self) -> Config {
        let file = File::create("config.json");
        let mut writer = BufWriter::new(file.unwrap());
        serde_json::to_writer_pretty(&mut writer, &self).expect("File Write Failed!");
        writer.flush().expect("Flush Failed!");
        self
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

fn get_all_comp_mods(conf: Config) -> Vec<PathBuf> {
    let mut mods = Vec::new();

    for paths in conf.storm_workshop_path.read_dir().unwrap() {
        //Terrible way of doing this but technically works, needs improved
        let temp_path = paths.unwrap().path().join("meshes");
        if temp_path.exists() {
            mods.push(temp_path);
        }
    }
    mods
}

fn main() {
    let mut config = Config::load();

    if config.validate_storm_dir() {
        println!("Valid");
        config.storm_workshop_path = craft_workshop_path(PathBuf::from(&config.storm_path));
        config = config.save();
    }
    else {
        println!("Not valid");
    }

    let mods = get_all_comp_mods(config);
    println!("Mods: {:?}", mods);
}
