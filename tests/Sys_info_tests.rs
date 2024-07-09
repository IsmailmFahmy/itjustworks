#![allow(dead_code)]
#![allow(unused)]
extern crate os_type;
use std::process::{Command, exit};
use std::path::Path;


// ----------------------- Public Functions -----------------------

pub fn get_sysinfo() -> SysInfo {
    SysInfo {
        distrobution : os_type::current_platform().os_type,
        init_system : check_init().unwrap(),
        dependencies_installed : check_dependencies()
    }
}




// ----------------------- Local Functions -----------------------


fn check_init() -> Result<InitSystem, String> {
    use std::collections::HashMap;

    // Hashmap of common init systems & their respective commands (of path to services for init systems that don't have commands)
    let init_systems : HashMap<InitSystem, &str> = HashMap::from([
    (InitSystem::Systemd, "systemctl"),
    (InitSystem::Sysvinit, "update-rc.d"),
    (InitSystem::S6, "/etc/s6"),
    (InitSystem::Openrc, "rc-update"),
    (InitSystem::Runit, "/etc/sv"),
    (InitSystem::Upstart, "start"),
    ]);

    
    for (system, command) in init_systems {
        // If the path to the services exists
        if system == InitSystem::S6 || system == InitSystem::Runit {
            if Path::new(command).exists() {
                return Ok(system)
            }
        }

        // For each init system, if it's respective command exists, then thats the init system
        if Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", command))
            .output()
            .expect("Failed to check for command")
            .status
            .success() == false {
                return Ok(system)
        }         
    }
    Err("Could not identify InitSystem".to_string())
}



fn check_dependencies() -> bool {
    // Array of dependencies
    let required_commands = ["gcc", "make", "pkg-config"];

    // if the command for this dependency doesn't exist, print an error
    for command in required_commands.iter() {
        if Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", command))
            .output()
            .expect("Failed to check for command")
            .status
            .success() == false
        {
            eprintln!("Error: Required command '{}' not found. Please install it and try again.", command);
            return false;
        }
    }
    true
}

// ----------------------- Structs -----------------------

pub struct SysInfo {
    distrobution : os_type::OSType,
    init_system : InitSystem,
    dependencies_installed : bool,

}



#[derive(Hash, PartialEq, Eq)]
pub enum InitSystem {
    Systemd,
    Openrc,
    Runit,
    S6,
    Sysvinit,
    Upstart
}
