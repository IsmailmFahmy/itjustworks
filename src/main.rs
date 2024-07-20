// #![allow(dead_code)]
// #![allow(unused)]

use tempdir::TempDir;

use std::io;
use std::env;

use itjustworks::packages::*;
use itjustworks::checks::SysInfo;
use itjustworks::extract::extract_package;
use itjustworks::download::download_files;
use itjustworks::install::compile::install_package;
use itjustworks::install::enable_service::enable_services;

fn main() -> Result<(), io::Error> {

    // ----------------- Testing packages -----------------
    let _libnotify = Package {
        name:               "libnotify",
        sourcecode_link:    "https://download.gnome.org/sources/libnotify/0.8/libnotify-0.8.3.tar.xz",
        extract_method:     ExtractMethodEnum::Tarxz,
        instal_method:      InstallMethodEnum::MakeInstall,
        service:            None,
    };

    let _cups = Package {
        name:               "cups",
        sourcecode_link:    "https://github.com/OpenPrinting/cups/archive/refs/heads/master.zip",
        extract_method:     ExtractMethodEnum::Zip,
        instal_method:      InstallMethodEnum::AutoGen,
        service:            Some("cups.service")
    };

    let _htop = Package {
        name:               "htop",
        sourcecode_link:    "https://github.com/htop-dev/htop/archive/refs/heads/main.zip",
        extract_method:     ExtractMethodEnum::Zip,
        instal_method:      InstallMethodEnum::AutoGen,
        service:            None
    };

    let _tmux = Package {
        name:               "tmux",
        sourcecode_link:    "https://github.com/tmux/tmux/releases/download/3.4/tmux-3.4.tar.gz",
        extract_method:     ExtractMethodEnum::Targz,
        instal_method:      InstallMethodEnum::AutoGen,
        service:            None
    };

    let _bluetooth = Package {
        name:               "bluez",
        sourcecode_link:    "http://www.kernel.org/pub/linux/bluetooth/bluez-5.66.tar.xz",
        extract_method:     ExtractMethodEnum::Tarxz,
        instal_method:      InstallMethodEnum::AutoGen,
        service:            Some("bluetooth.service")
    };

    // ----------------------------------------------------


    // todo!("select_packages using ratatui");
    let selected : Vec<Package> = vec![];

    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "ItJustWorks".
    let tmp_dir = TempDir::new("ItJustWorks").expect("could not create temporary directory");

    let sysinfo = SysInfo::new(); // Defining initsystem / checking dependencies

    for package in selected {

        // download each package                         link                 path
        let downloaded_file_path = download_files(package.sourcecode_link , &tmp_dir)
            .expect(format!("Failed to download the {} package", package.name).as_str());

        let working_path = tmp_dir.path();
        let _ = env::set_current_dir(&working_path);    // Change working directory to the temporary directory

        extract_package(&package.extract_method, &downloaded_file_path, &working_path.join(package.name))
            .expect(format!("Failed to extract the {} package", package.name).as_str());

        
        let _ = env::set_current_dir(&working_path.join(package.name));     //change directory to the extracted package

        // pause();
        install_package(&package.instal_method)
            .expect(format!("Failed to install the {} package", package.name).as_str());

        // unwrap potential error if it returns any
        if let Some(service) = package.service {
        enable_services(&service, &sysinfo)
            .expect(format!("Failed to enable service for {}", package.name).as_str());
        }
    };
    Ok(())
}









// read stdin to pause execution for debugging
#[allow(dead_code)]
fn pause() {
    print!("Pausing... Please press ENTER to continue excecution");
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    print!("\n")
}
