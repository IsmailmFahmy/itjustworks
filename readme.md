Ever wanted a simple linux install to have all the basic functionality working without installing and configuring every package individually? Well this is where this script comes in to rescue the day.

This Rust project is aimed to take a headless fresh install and configure basic elements like audio and printers.

This project is partially inspired by Chris Titus's Wintool.\





# What To Configure
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
## Later
- firewall
- backup

# Things To Tackle

# Cli Tooling
[**Ratatui**](https://ratatui.rs/introduction/)

## How to make it compatible with all distros

Copy the repo, sudo make install


How linuxfromscratch does it.

# Package installation

## Package configuration

## Hardware detection


# Dependencies
- git
- build essencials
	- GCC
	- Make


---
# Future plans
- Make compatible with toml config file
