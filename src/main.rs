use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Lines, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, Builder},
    time::Duration,
};

use http::{PostEvent, PutEvent, Response};
use serde_json::{json, Value};
use storage::{
    db::{ResponseUser, UserLogin},
    register_new_user,
};

use crate::http::{Accept, ConnectionType, IpAdress, Request};

mod http;
mod server;
mod storage;

const STATUS_OK: &'static str = "HTTP/1.1 200 OK";
const STATUS_GONE: &'static str = "HTTP/1.1 410 Gone";
const WEBSITE_PATH: &'static str = "./website/";
const DATABASE_PATH: &'static str = "./database/";

fn main() -> io::Result<()> {
    let ip = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap();
    let pool = ThreadPool::new(10)?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| match handle_connection(stream) {
            Ok(o) => {}
            Err(e) => println!("{e}"),
        });
    }
    Ok(())
}

fn tokenize_response(mut lines: Vec<String>) -> io::Result<Response> {
    if lines.is_empty() {
        return Err(io::ErrorKind::InvalidData.into());
    }

    for l in &lines {
        println!("{l}");
    }
    let request_line = lines[0].clone();

    let req_start_index = 0;
    let req_end_index = request_line.find('/').unwrap() - 1;
    let http_start = request_line.find("HTTP/").unwrap();

    let req_txt = &request_line[req_start_index..req_end_index].trim();
    let file_txt = request_line[(req_end_index + 2)..http_start - 1].trim();
    let request = Request::new(req_txt);

    let mut ip = IpAdress::default();
    let mut connection_type = ConnectionType::Error;
    let mut accept = Accept::default();
    let mut body = false;
    let mut body_txt = String::new();
    for l in lines {
        if l.contains("Host:") {
            let ip_txt_len = l.len();
            let ip_start_index = l.clone().find(':').unwrap() + 1;

            let xx = &l[ip_start_index..ip_txt_len];
            let ip_end_index = xx.find(':').unwrap() + ip_start_index;

            let ip_adress_txt = &l[ip_start_index..ip_end_index];

            let port_txt = &l[ip_end_index + 1..l.len()];

            ip = IpAdress::from_str(ip_adress_txt.trim(), port_txt.trim());
            continue;
        }

        if l.contains("Accept") {
            let start_index = l.clone().find(':').unwrap() + 1;
            accept = Accept::new(&l[start_index..l.len()]);
            continue;
        }

        if l.contains("Connection:") {
            connection_type = ConnectionType::new(&l[&l.find(':').unwrap() + 1..l.len()]);
            continue;
        }
        if l.trim().is_empty() {
            body = true;
            continue;
        }
        if body == true {
            body_txt.push_str(l.as_str());
            continue;
        }
    }
    let mut b: Option<String> = None;
    if body {
        b = Some(body_txt);
    }

    Ok(Response {
        requested_file_path: file_txt.to_string(),
        request,
        host_ip: ip,
        accept_ext: accept,
        connection_type,
        fetch_type: None,
        fetch_mode: None,
        fetch_site: None,
        body: b,
    })
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let http_request: Vec<_>;
    {
        let mut buffer = [0u8; 4096];

        let bytes_read = stream.read(&mut buffer).unwrap();
        let s = String::from_utf8(buffer[0..bytes_read].to_vec()).unwrap();
        http_request = s.split("\n").map(|f| f.to_string()).collect();
    }
    let req = tokenize_response(http_request)?;
    let mut filename = req.requested_file_path.clone();
    let mut response_lines = vec![];
    match req.request {
        Request::Get => {
            if req.requested_file_path.is_empty() {
                filename = "index.html".to_string();
            } else {
                if req.accept_ext.html == true {
                    filename = format!("{}{}", req.requested_file_path, ".html");
                }
            }

            let contents = match fs::read_to_string(format!("{}{}", WEBSITE_PATH, filename)) {
                Ok(file) => file,
                Err(e) => {
                    send_gone(&mut stream)?;
                    return Err(e);
                }
            };

            let index = filename.find(".");
            let mut content_type = "e";
            match index {
                Some(i) => {
                    let extension = &filename[i + 1..filename.len()];
                    if extension == "html" {
                        content_type = "Content-Type: text/html";
                    } else if extension == "js" {
                        content_type = "Content-Type: application/javascript";
                    }
                }
                None => {
                    println!("ERROR? {}", filename);
                }
            }

            let content_length = format!("Content-Length: {}", contents.len());

            response_lines.push(STATUS_OK);
            response_lines.push(content_type);
            response_lines.push(content_length.as_str());
            response_lines.push("");
            response_lines.push(contents.as_str());

            let response = response_lines.join("\n");
            stream.write_all(response.as_bytes())?;
        }

        Request::Head => todo!(),
        Request::Post => {}
        Request::Put => {
            let info: Value = serde_json::from_str(req.body.unwrap().as_str()).unwrap();
            let command = info["request"].as_str().unwrap();

            match command {
                "register" => {
                    let firstname = info["firstname"].as_str().unwrap();
                    let lastname = info["lastname"].as_str().unwrap();
                    let username = info["username"].as_str().unwrap();
                    let password = info["password"].as_str().unwrap();

                    match register_new_user(firstname, lastname, username, password) {
                        Ok(_) => {
                            // SEND OK REQUEST
                            let response_json = json!({"request": "ok", "hello":"1"}).to_string();
                            let response = http::json_response(response_json);

                            stream.write(response.as_bytes()).unwrap();
                        }
                        Err(_) => {
                            let response_json = json!({"request": "username exist"}).to_string();
                            let response = http::json_response(response_json);
                            stream.write(response.as_bytes());
                        }
                    }
                }
                _ => {}
            }

            // match info {
            //     Ok(x) => {
            //         match register_new_user(x.firstname, x.lastname, x.username, x.password){
            //             Ok(_) => {
            //                 // SEND OK REQUEST
            //                 stream.write(json!({"Request": "ok"}).to_string().as_bytes());
            //             },
            //             Err(_) => {
            //                 stream.write(json!({"Request": "username exist"}).to_string().as_bytes());
            //             },
            //         }
            //     },
            //     Err(_) => todo!(),
            // }
        }
        Request::Delete => todo!(),
        Request::Connect => todo!(),
        Request::Options => todo!(),
        Request::Trace => todo!(),
        Request::Patch => todo!(),
    }

    Ok(())
}

fn send_gone(stream: &mut std::net::TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 410 Gone\nContent-Type: text/plain";

    stream.write_all(response.as_bytes())?;
    Ok(())
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> io::Result<ThreadPool> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))?);
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> io::Result<Worker> {
        let builder = Builder::new();

        let thread = builder.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            job();
        })?;

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }
}
