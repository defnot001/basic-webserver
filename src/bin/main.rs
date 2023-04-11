use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use basic_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_home_request = b"GET / HTTP/1.1\r";
    let get_sleep_request = b"GET /sleep HTTP/1.1\r";

    let (status_line, filename) = if buffer.starts_with(get_home_request) {
        ("HTTP/1.1 200 OK", "pages/index.html")
    } else if buffer.starts_with(get_sleep_request) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "pages/sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
