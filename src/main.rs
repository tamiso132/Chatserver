use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
pub mod relational;
mod server_api;
mod storage;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let received_data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        if check_valid(&received_data) {
            // Code matches, handle the connection
            println!("Accepted connection with valid code: {}", received_data);
            // Respond to the client
            stream.write_all(b"Valid code received\n").unwrap();
            // Implement your logic to process the connection
        } else {
            // Invalid code, close the connection
            println!("Rejected connection with invalid code: {}", received_data);
            return;
        }
    } else {
        eprintln!("Error reading from stream");
    }
}

fn main() -> std::io::Result<()> {
    // relational::bench_test();
    // thread::spawn(move || {
    //     for (stream) in listener.incoming() {
    //         match stream {
    //             Ok(stream) => {
    //                 println!("Connected");
    //                 // Spawn a new thread to handle the client connection
    //                 thread::spawn(move || {
    //                     handle_client(stream);
    //                 });
    //             }
    //             Err(e) => {
    //                 eprintln!("Error accepting connection: {}", e);
    //             }
    //         }
    //     }
    // });

    // Keep the main thread running indefinitely
    // let mut stream = TcpStream::connect("37.208.27.16:2000").expect("error");
    // if let Err(e) = stream.write_all("no".as_bytes()) {
    //     eprintln!("Error sending code {}", e);
    // }

    // let mut response = String::new();
    // if let Err(e) = stream.read_to_string(&mut response) {
    //     eprintln!("Error receiving response: {}", e);
    // }
    loop {}
}

fn check_valid(s: &String) -> bool {
    let file = File::open("validcode/valid.txt").expect("file not found");
    let reader = BufReader::new(file);

    // Iterate through each line in the file
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if s == &l {
                    return true;
                }
            }
            Err(_) => {}
        }
    }
    return false;
}
