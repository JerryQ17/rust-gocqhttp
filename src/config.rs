use crate::Result;
use log::error;
use reqwest::Client;
use serde::Deserialize;
use serde_yaml::{from_reader, from_value, Value};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Debug)]
pub struct Server {
    pub http: Vec<Http>,
    pub http_reverse: Vec<HttpReverse>,
    pub lambda: Vec<Lambda>,
    pub ws: Vec<Ws>,
    pub ws_reverse: Vec<WsReverse>,
    pub http_client: Client,
    pub ws_client: Vec<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpReverse {
    url: String,
    secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Lambda {
    type_: String,
}

#[derive(Debug, Deserialize)]
pub struct Ws {
    address: String,
}

#[derive(Debug, Deserialize)]
pub struct WsReverse {
    universal: String,
    api: String,
    event: String,
}

fn gen_error(msg: &str) -> Error {
    error!("{}", msg);
    Error::new(
        ErrorKind::InvalidData,
        format!("{}，请参考https://docs.go-cqhttp.org/guide/config.html#%E9%85%8D%E7%BD%AE%E4%BF%A1%E6%81%AF", msg)
    )
}

impl Server {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let content = from_reader::<File, Value>(file)?;
        let servers = content
            .get("servers")
            .ok_or(gen_error("在config.yml配置文件中未找到servers字段"))?
            .as_sequence()
            .ok_or(gen_error("servers字段配置错误"))?;
        let mut config = Server {
            http: Vec::new(),
            http_reverse: Vec::new(),
            lambda: Vec::new(),
            ws: Vec::new(),
            ws_reverse: Vec::new(),
            http_client: Client::new(),
            ws_client: Vec::new(),
        };
        for item in servers {
            let m = item.as_mapping().ok_or(gen_error("servers字段配置错误"))?;
            for (server_type, server) in m {
                let name = server_type
                    .as_str()
                    .ok_or(gen_error("servers字段配置错误"))?;
                match name {
                    "http" => {
                        config.http.push(from_value(server.clone())?);
                        if let Some(r_http) = server.get("post") {
                            let seq = r_http
                                .as_sequence()
                                .ok_or(gen_error("servers.http.post字段类型错误"))?;
                            for item in seq {
                                config.http_reverse.push(from_value(item.clone())?);
                            }
                        }
                    }
                    "lambda" => {
                        config.lambda.push(from_value(server.clone())?);
                    }
                    "ws" => {
                        let ws = from_value::<Ws>(server.clone())?;
                        let stream = connect_async(ws.address.as_str()).await?.0;
                        config.ws.push(ws);
                        config.ws_client.push(stream);
                    }
                    "ws-reverse" => {
                        config.ws_reverse.push(from_value(server.clone())?);
                    }
                    _ => {}
                }
            }
        }
        Ok(config)
    }
}
