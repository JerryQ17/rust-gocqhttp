mod config;
mod error;
mod message;

use crate::config::Server;
use crate::error::Result;
use log::{error, info};
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
            error!("{}不是一个文件夹", directory);
            return Err(Box::new(Error::new(ErrorKind::NotFound, "未找到文件夹")));
        }
        match Server::from_file(path.join("config.yml")).await {
            Ok(server) => {
                info!("读取配置文件成功");
                Ok(Self {
                    directory,
                    process: None,
                    server,
                })
            }
            Err(e) => {
                error!("读取配置文件失败: {}", e);
                Err(e)
            }
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.process.is_none() {
            info!("启动go-cqhttp");
            self.process = Some(
                Command::new("go-cqhttp.bat")
                    .current_dir(&self.directory)
                    .spawn()?,
            );
        } else {
            info!("go-cqhttp已经启动");
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        match &mut self.process {
            None => {
                info!("go-cqhttp未启动");
                Ok(())
            }
            Some(ref mut p) => match p.kill() {
                Ok(_) => {
                    self.process = None;
                    info!("go-cqhttp已停止");
                    Ok(())
                }
                Err(e) => {
                    error!("go-cqhttp停止失败: {}", e);
                    Err(Box::new(e))
                }
            },
        }
    }
}
