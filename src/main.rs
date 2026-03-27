use std::env::current_dir;
use std::path::PathBuf;

struct config {
    storm_path: PathBuf,
    storm_workshop_path: PathBuf,
    backup_path: PathBuf,
    output_path: PathBuf,
}

impl config {
    fn new(current_dir: PathBuf) -> config {
        config {
            storm_path: PathBuf::new(),
            storm_workshop_path: PathBuf::new(),
            backup_path: current_dir.clone(),
            output_path: current_dir,
        }
    }

    fn save(self) {
        
    }
}

fn main() {
    let cwd = current_dir();

    let mut conf = config::new(cwd.unwrap_or(PathBuf::new()));
}
