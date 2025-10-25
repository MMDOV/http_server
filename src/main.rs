use http_net::ThreadPool;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines().map(|line| line.unwrap());
    let request_line = lines.next().unwrap();
    let headers: Vec<_> = lines.take_while(|line| !line.is_empty()).collect();

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Hello World!"),
        "GET /mamad HTTP/1.1" => ("HTTP/1.1 200 OK", "Hi I'm mamad"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "Hello World!")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404 NOT FOUND"),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {request_line:#?}");
    println!("Headers: {headers:#?}");
}
