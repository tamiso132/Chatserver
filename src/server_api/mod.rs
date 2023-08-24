use std::{collections::HashMap, error::Error, sync::Arc};

use serde_derive::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
};

use crate::relational::TableInfo;
#[derive(Serialize, Deserialize, Clone, Copy)]
enum Event {
    Add = 0,
    Remove = 1,
    Update = 2,
}
impl Event {
    pub fn buffer_size() -> u8 {
        1
    }
    pub fn to_u8(e: &Event) -> u8 {
        *e as u8
    }
    pub fn to_enum(e: u8) -> Option<Event> {
        match e {
            0 => Some(Event::Add),
            1 => Some(Event::Remove),
            2 => Some(Event::Update),
            _ => None,
        }
    }
}

async fn connect_to_server(secret: String) -> Result<TcpStream, Box<dyn Error>> {
    let mut stream = TcpStream::connect("37.208.27.16:2000").await?;
    stream.write_all(secret.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    Ok(stream)
}

async fn listen_stream(stream: &mut ReadHalf<TcpStream>) {
    let mut info_buffer: Vec<u8> =
        vec![0; (TableInfo::buffer_size() + Event::buffer_size() as u16) as usize];
    loop {
        match stream.read(&mut info_buffer).await {
            Ok(0) => {}
            Ok(_) => match Event::to_enum(info_buffer[0]) {
                Some(ev) => match ev {
                    Event::Add => todo!(),
                    Event::Remove => todo!(),
                    Event::Update => todo!(),
                },
                None => todo!(),
            },
            Err(e) => {
                eprintln!("Error reading response: {}", e);
            }
        }
    }
}

async fn write_stream(stream: &mut WriteHalf<TcpStream>) {}

async fn run() {
    match connect_to_server("hellokitty".to_string()).await {
        Ok(stream) => {
            let (mut read_s, mut write_s) = tokio::io::split(stream);
            tokio::spawn(async move { listen_stream(&mut read_s).await });
            tokio::spawn(async move { write_stream(&mut write_s).await });
        }
        Err(e) => eprintln!("Unable to connect to server\n{e}"), // TODO some kind of thread sleep + repeat
    }
}

async fn lek() {}
