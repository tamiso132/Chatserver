use std::{
    fs,
    io::{self, BufRead, BufReader, Lines, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, Builder},
    time::Duration,
};

use http::{PostEvent, PutEvent, Response};

use crate::http::{Accept, ConnectionType, IpAdress, Request};

mod http;
mod server;
mod storage;

const WEBSITE_PATH: &'static str = "./website/";
const HEADER: u16 = 0x170 << 8 | 0x170;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("192.168.0.107:7878").unwrap();
    let pool = ThreadPool::new(10)?;
    println!("ip: 192.168.0.107:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| match handle_connection(stream) {
            Ok(o) => println!("good"),
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
        println!("print: {}", &l);
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

        stream.read(&mut buffer).unwrap();
        let s = String::from_utf8(buffer.to_vec()).unwrap();
        http_request = s.split("\n").map(|f| f.to_string()).collect();

        // http_request = buf_reader
        //     .lines()
        //     .map(|result| result.unwrap())
        //     .take_while(|line| !line.is_empty())
        //     .collect();
    }
    let req = tokenize_response(http_request)?;
    let mut filename = req.requested_file_path.clone();
    let status = "HTTP/1.1 200 OK";
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
                    let response = "HTTP/1.1 410 Gone\nContent-Type: text/plain";
                    stream.write_all(response.as_bytes())?;
                    return Err(e);
                }
            };
            let length = contents.len();
            let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes())?;
        }

        Request::Head => todo!(),
        Request::Post => {
            if req.requested_file_path.parse::<u8>().unwrap() == PostEvent::Login as u8 {
                let s = &req.body.unwrap();
                let user = s[0..50].trim();
                let password = s[50..s.len()].trim();
                println!("{}\n{}", user, password);
            }
        }
        Request::Put => {
            if req.requested_file_path.parse::<u8>().unwrap() == PutEvent::Register as u8 {}
        }
        Request::Delete => todo!(),
        Request::Connect => todo!(),
        Request::Options => todo!(),
        Request::Trace => todo!(),
        Request::Patch => todo!(),
    }

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
