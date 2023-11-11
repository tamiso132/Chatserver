use std::{
    fs,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread::{self, Builder},
};

use database::ThreadPool;

pub mod relational;
mod server;
mod storage;

const WEBSITE_PATH: &'static str = "./website/";
const HEADER: u16 = 0x170 << 8 | 0x170;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10)?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);

    println!("HOW HOW HOW\n\n");

    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    let accept_header = lines
        .find(|line| line.as_ref().unwrap().starts_with("Accept: "))
        .unwrap()
        .unwrap();

    let media_types: Vec<&str> = accept_header[8..].split(',').collect();
    let mut extension = "";
    for media_type in media_types {
        if media_type == "text/html" {
            extension = ".html";
            break;
        }
    }

    if request_line.contains("GET /") {
        let (status_line, filename) = if request_line.contains("GET /") {
            let http_byte_start = request_line.find("HTTP").unwrap();

            let start_bytes = "GET /".to_string().len();
            println!("HERE: {}", request_line);
            let file_name = &request_line[start_bytes..http_byte_start - 1];
            println!("Filename: {}", file_name.len());

            if file_name.is_empty() {
                ("HTTP/1.1 200 OK", "index")
            } else {
                ("HTTP/1.1 200 OK", file_name)
            }
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let website_path = format!("{}{}{}", WEBSITE_PATH, filename, extension);

        println!("{}", website_path);

        let contents = fs::read_to_string(website_path).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
    }

    Ok(())
}

enum Request {
    GET,  //
    HEAD, //
    POST,
    PUT,
    DELETE, //
    TRACE,  //
    PATCH,
}

#[derive(Default)]
struct IpAdress {
    ip: [u8; 4],
    port: u16,
}

impl IpAdress {
    fn print(&self) {
        println!(
            "{}.{}.{},{}:{}",
            self.ip[0], self.ip[1], self.ip[2], self.ip[3], self.port
        );
    }
}

struct HtmlResponse {
    request: Request,
}
