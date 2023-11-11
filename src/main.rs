use std::{
    fs,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, Builder},
};

use proc_macro::WhoAmI;

#[derive(WhoAmI)]
struct Point {
    x: f64,
    y: f64
}


pub mod relational;
mod server;
mod storage;

const WEBSITE_PATH: &'static str = "./website/";

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

        let contents:String = match fs::read_to_string(website_path){
            Ok(x) => {x},
            Err(e) => {
            let response = "HTTP/1.1 410 Gone\nContent-Type: text/plain";
                stream.write_all(response.as_bytes()).unwrap();
                return Err(e)
            },
        };
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
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
            println!("Shutting down worker {}", worker.id);

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

            println!("Worker {id} got a job; executing.");

            job();
        })?;

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }
}
