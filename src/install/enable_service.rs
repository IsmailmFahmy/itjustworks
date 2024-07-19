use crate::checks::{InitSystem, SysInfo};
use crate::utils::cmd;

use InitSystem::*;

pub fn enable_services(service : &str, sysinfo: &SysInfo) -> Result<String, String> {

        let output = match sysinfo.init_system {
            Systemd => cmd(format!("sudo systemctl enable {service} && sudo systemctl start {service}").as_str()),
            Openrc => cmd(format!("sudo rc-update add {service} default && sudo rc-service {service} start").as_str()),
            Runit => cmd(format!("ln -s /etc/sv/{service} /var/service/ && sudo sv start {service}").as_str()),
            S6 => cmd(format!("ln -s /etc/s6/{service} /etc/s6/service/ && sudo s6-svc -u /etc/s6/service/{service}").as_str()),
            Sysvinit => cmd(format!("sudo update-rc.d {service} defaults && sudo service {service} start").as_str()),
            Upstart => cmd(format!("sudo start {service}").as_str()),
        };
        return output;
}
