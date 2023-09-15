mod config;
mod error;

use crate::error::Result;
use reqwest::Client;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{Child, Command};

pub struct GoCqhttp {
    directory: String,
    process: Option<Child>,
    client: Client,
}

impl GoCqhttp {
    pub fn new(directory: String) -> Result<Self> {
        if Path::new(&directory).is_dir() {
            return Err(Box::new(Error::new(ErrorKind::NotFound, "未找到该目录")));
        }
        Ok(Self {
            directory,
            process: None,
            client: Client::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        if self.process.is_none() {
            self.process = Some(
                Command::new("go-cqhttp.bat")
                    .current_dir(&self.directory)
                    .spawn()?,
            );
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        match &mut self.process {
            None => Ok(()),
            Some(ref mut p) => match p.kill() {
                Ok(_) => {
                    self.process = None;
                    Ok(())
                }
                Err(e) => Err(Box::new(e)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
