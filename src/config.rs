use crate::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_yaml::{from_reader, from_value, Value};
use std::fs::File;
use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Debug)]
struct Server {
    pub http: Vec<Http>,
    pub http_reverse: Vec<HttpReverse>,
    pub lambda: Vec<Lambda>,
    pub ws: Vec<Ws>,
    pub ws_reverse: Vec<WsReverse>,
    http_client: Client,
    ws_client: Vec<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[derive(Debug, Deserialize)]
struct Http {
    address: String,
}

#[derive(Debug, Deserialize)]
struct HttpReverse {
    url: String,
    secret: String,
}

#[derive(Debug, Deserialize)]
struct Lambda {
    type_: String,
}

#[derive(Debug, Deserialize)]
struct Ws {
    address: String,
}

#[derive(Debug, Deserialize)]
struct WsReverse {
    universal: String,
    api: String,
    event: String,
}

impl Server {
    fn gen_error(msg: &str) -> Error {
        Error::new(
            ErrorKind::InvalidData,
            format!("{}，请参考https://docs.go-cqhttp.org/guide/config.html#%E9%85%8D%E7%BD%AE%E4%BF%A1%E6%81%AF", msg)
        )
    }

    pub async fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let content = from_reader::<File, Value>(file)?;
        let servers = content
            .get("servers")
            .ok_or(Self::gen_error("在config.yml中未找到servers字段"))?
            .as_sequence()
            .ok_or(Self::gen_error("servers字段配置错误"))?;
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
            let m = item
                .as_mapping()
                .ok_or(Self::gen_error("servers字段配置错误"))?;
            for (server_type, server) in m {
                let name = server_type
                    .as_str()
                    .ok_or(Self::gen_error("servers字段配置错误"))?;
                match name {
                    "http" => {
                        let http = from_value::<Http>(server.clone())?;
                        config.http.push(http);
                        match server.get("post") {
                            None => {}
                            Some(seq) => {
                                let seq = seq
                                    .as_sequence()
                                    .ok_or(Self::gen_error("post字段类型错误"))?;
                                for item in seq {
                                    let post = from_value::<HttpReverse>(item.clone())?;
                                    config.http_reverse.push(post);
                                }
                            }
                        }
                    }
                    "Lambda" => {
                        let lambda = from_value::<Lambda>(server.clone())?;
                        config.lambda.push(lambda);
                    }
                    "Ws" => {
                        let ws = from_value::<Ws>(server.clone())?;

                        let stream = connect_async(ws.address.as_str()).await?.0;
                        config.ws.push(ws);
                        config.ws_client.push(stream);
                    }
                    "WsReverse" => {
                        let ws_reverse = from_value::<WsReverse>(server.clone())?;
                        config.ws_reverse.push(ws_reverse);
                    }
                    _ => return Err(Box::new(Self::gen_error("servers字段配置错误"))),
                }
            }
        }
        Ok(config)
    }
}
