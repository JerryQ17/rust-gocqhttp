mod config;
mod error;
mod message;

use crate::config::Server;
use crate::error::Result;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{Child, Command};

pub struct GoCqhttp {
    directory: String,
    process: Option<Child>,
    server: Server,
}

impl GoCqhttp {
    pub async fn new(directory: String) -> Result<Self> {
        let path = Path::new(&directory);
        if !path.is_dir() {
            return Err(Box::new(Error::new(ErrorKind::NotFound, "未找到该文件夹")));
        }
        let server = Server::from_file(path.join("config.yml")).await.unwrap();
        Ok(Self {
            directory,
            process: None,
            server,
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
