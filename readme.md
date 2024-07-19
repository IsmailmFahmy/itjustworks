Ever wanted a simple Linux install to have all the basic functionality working without installing and configuring every package individually? Well this is where this script comes in to rescue the day.

This Rust project is aimed to take a headless fresh install and configure basic elements like audio and printers.

This project is partially inspired by Chris Titus's Wintool.\




# How It Just Works
1. Check dependencies
1. Identify init system
1. User selects desired package(s) using a TUI
2. Selected package(s) are added to a vector
3. package is downloaded from source
4. package is de-compressed
5. package is installed (make install / etc)
6. service is enabled if the package has any


# Things To Tackle
- [x] Checking Dependencies
- [x] Identify Init system
	- [x] Openrc,
    - [x] Runit,
    - [x] S6,
    - [x] Sysvinit,
    - [x] Upstart
    - [x] Systemd
- [x] Download Package
- [x] Extract Package
	- [x] Zip
	- [x] Tar.gz
	- [x] Tar.xz
- [x] Compile Package
	- [x] GCC
- [x] Install services
- [x] 
- [ ] [**Ratatui** for TUI](https://ratatui.rs/introduction/)
- [ ] Logging
- [ ] Proper error handling


# Planning to support Packages for 
- Wifi
- Bluetooth
- Printers
- Codecs
- Audio
- Power Management
- Nvidia Drivers
- Notifications
- Flatpacks
- NTP
- Clipboard
- Brightness control
- Backlight
- Firewall
- Backup


# Dependencies
- git
- build essencials
	- GCC
	- Make




---
# Future plans
- Make compatible with toml config file
- Detect printers
- Detect nvidia gpu
