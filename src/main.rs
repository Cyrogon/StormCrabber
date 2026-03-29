use std::{env, fs, io};
use std::env::current_dir;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;
use zip_extensions::zip_writer::zip_create_from_directory;

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
    if path.exists() {
        path
    } else {
        PathBuf::new()
    }

}

fn contains_mod(path: &PathBuf) -> bool {
    let mut isdir = false;

    for entry in fs::read_dir(path).unwrap() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        if metadata.is_dir() && !entry.path().ends_with("playlist") {
            isdir = true;
            break;
        };
    };
    isdir
}
fn get_all_comp_mods(conf: &Config) -> Vec<PathBuf> {
    let mut mods = Vec::new();

    for paths in conf.storm_workshop_path.read_dir().unwrap() {

        let temp = match paths {
            Ok(p) => p,
            Err(_) => continue,
        };

        if contains_mod(&temp.path()) {
            mods.push(temp.path());
        }
    }
    mods
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}


fn mods_prep(mods: Vec<PathBuf>, config: &Config) {
    for dirs in mods {
        copy_dir_all(dirs.as_path(), config.output_path.as_path()).expect("File Write Failed!");
    }
}

fn zip_mods(config: &Config) {
    zip_create_from_directory(&config.output_path, &current_dir().unwrap()).expect("Archiving Failed");
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

    let mods = get_all_comp_mods(&config);
    println!("Mods: {:?}", mods);
    mods_prep(mods, &config);
    zip_mods(&config);
}