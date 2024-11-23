use std::error::Error;
use std::fs::File;
use std::path::Path;
use sysinfo::{System};

pub fn watch_denis_service() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let target_service = "denis.service";
    for (pid, process) in sys.processes() {
        if let Some(name) = process.name().to_str() {
            if name.contains(target_service) {
                println!(
                    "Servis: {}, PID: {}, RAM Kullanımı: {} KB",
                    name,
                    pid,
                    process.memory()
                );
            }
        }
    }
}



pub fn create_denis_properties() -> Result<(), Box<dyn Error>> {
    let properties_file = "denis.properties";
    let path = Path::new(properties_file);

    if !path.exists() {
        File::create(path)?;
        if path.exists() {
            println!("denis.properties created.");
        } else {
            println!("denis.properties cannot be created.");
        }
    } else {
        println!("denis.properties already exists.");
    }
    Ok(())
}