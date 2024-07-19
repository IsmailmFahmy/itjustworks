use std::io;
use std::io::{ErrorKind, Error};
use std::process::{Output,Command};

pub fn cmd(command: &str) -> Result<String, io::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(Output { status, stdout, stderr }) => {
            if status.success() {
                Ok(String::from_utf8_lossy(&stdout).into_owned())
            } else {
                let error_message = String::from_utf8_lossy(&stderr).into_owned();
                Err(Error::new(ErrorKind::Other, error_message))
            }
        }
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}
