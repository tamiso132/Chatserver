use std::{default, net::TcpStream};

use serde_json::json;

pub struct Response {
    pub requested_file_path: String,
    pub request: Request,
    pub host_ip: IpAdress,
    pub accept_ext: Accept,
    pub connection_type: ConnectionType,
    pub body: Option<String>,
    pub fetch_type: Option<FetchType>,
    pub fetch_mode: Option<FetchMode>,
    pub fetch_site: Option<FetchSite>,
}

#[repr(u8)]
pub enum Request {
    Get = 0,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}
impl Request {
    pub fn new(txt: &str) -> Self {
        let request: Request = match txt {
            "GET" => Request::Get,
            "HEAD" => Request::Head,
            "POST" => Request::Post,
            "PUT" => Request::Put,
            "DELETE" => Request::Delete,
            "CONNECT" => Request::Connect,
            "OPTIONS" => Request::Options,
            "TRACE" => Request::Trace,
            "PATCH" => Request::Patch,
            _default => {
                panic!("Request type missing")
            }
        };
        request
    }
}

pub struct IpAdress {
    ip_adress: [u8; 4],
    port: u16,
}

impl IpAdress {
    pub fn from_str(ip_txt: &str, port_txt: &str) -> Self {
        let mut s = String::new();
        let mut ip_index = 0;
        let mut ip_adress: [u8; 4] = [0; 4];
        let mut port: u16 = 0;

        for e in ip_txt.chars() {
            if e == '.' {
                ip_adress[ip_index] = s.parse::<u8>().unwrap();
                ip_index += 1;
                s.clear();
                continue;
            }

            s.push(e);
        }
        ip_adress[ip_index] = s.parse::<u8>().unwrap();
        port = port_txt.to_string().parse::<u16>().unwrap();

        Self { ip_adress, port }
    }

    pub fn default() -> Self {
        Self {
            ip_adress: Default::default(),
            port: Default::default(),
        }
    }

    pub fn print(&self) {
        let ip_txt = format!(
            "{}.{}.{}.{}:{}",
            self.ip_adress[0], self.ip_adress[1], self.ip_adress[2], self.ip_adress[3], self.port
        );
        println!("{}", ip_txt);
    }
}

pub enum PostEvent {
    Login = 0,
}

pub enum PutEvent {
    Register = 0,
}

pub struct Accept {
    pub html: bool,
    pub xhtml: bool,
    pub xml: bool,
    pub avif_image: bool,
    pub webp_image: bool,
}
impl Accept {
    pub fn new(line: &str) -> Self {
        let accept_types: Vec<_> = line[0..].split(',').collect();

        let mut html = false;
        let mut xhtml = false;
        let mut xml = false;
        let mut avif_image = false;
        let mut webp_image = false;
        for e in accept_types {
            match e.trim() {
                "text/html" => html = true,
                "application/xhtml+xml" => xhtml = true,
                "application/xml;q=0.9" => xml = true,
                "image/avif" => avif_image = true,
                "image/webp" => webp_image = true,
                "*/*;q=0.8" => {}
                x => {}
            }
        }
        Accept {
            html,
            xhtml,
            xml,
            avif_image,
            webp_image,
        }
    }
    pub fn default() -> Self {
        Self {
            html: false,
            xhtml: false,
            xml: false,
            avif_image: false,
            webp_image: false,
        }
    }
}

pub enum ConnectionType {
    Error,
    KeepAlive, // continues to be open after request
    Close,     // the socket closes after this request
    Upgrade,   // upgrades into Websocket
}

impl ConnectionType {
    pub fn new(line: &str) -> Self {
        let connection = match line.trim() {
            "keep-alive" => ConnectionType::KeepAlive,
            "upgrade" => ConnectionType::Upgrade,
            "close" => ConnectionType::Close,
            x => panic!("undefined connection type: {}", x),
        };
        connection
    }
}

pub enum FetchType {
    Empty,    // empty document
    Object,   // plugin resource
    Manifest, // web app manifest file
    Document,
    Font,
    Image,
    Media,  // audio or video
    Script, // javascript/typescript
    Style,  // css
    Worker, // service worker script
}

impl FetchType {
    fn new(line_txt: &str) -> Self {
        match line_txt {
            "empty" => FetchType::Empty,
            "object" => FetchType::Object,
            "document" => FetchType::Document,
            "font" => FetchType::Font,
            "image" => FetchType::Image,
            "media" => FetchType::Media,
            "script" => FetchType::Script,
            "style" => FetchType::Style,
            "worker" => FetchType::Worker,
            x => panic!("undefined: {}", x),
        }
    }
}

///
/// The mode which the request is being made
pub enum FetchMode {
    Navigate,   // the request is made as part of navigation
    SameOrigin, // the request is made same origin(domain)
    NoCors,     // request is made cross origin request
}
///  relationship between origin of the request and the origin of the destination
pub enum FetchSite {
    SameOrigin,
    SameSite,
    CrossOrigin,
}

pub fn json_response(json_data: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json_data.len(),
        json_data
    )
}

pub fn ok_code() -> String {
    let json = json!({"request": "ok"}).to_string();
    format!(
        "HTTP/1.1 200 OK\nContent-Length: {}\n\n{}",
        json.len(),
        json
    )
}
