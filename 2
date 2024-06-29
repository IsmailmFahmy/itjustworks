extern crate os_type;
use std::process::{Command, exit};


struct SysInfo {
    distrobution : os_type::OSType,
    init_system : InitSystem,
    dependencies_installed : bool,

}

pub fn testie() {
    let sysinfo = SysInfo {
        distrobution : os_type::current_platform().os_type,
        init_system : InitSystem::S6,
        dependencies_installed : true
    };
    println!("{:?}", sysinfo.distrobution)
}


enum InitSystem {
    Systemd,
    Openrc,
    Runit,
    S6,
    Sysvinit
}


fn check_dependencies() {
    let required_commands = ["gcc", "make", "pkg-config"];

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
            exit(1);
        }
    }
}

