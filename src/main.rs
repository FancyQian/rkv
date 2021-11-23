
mod rkv;

use std::{net::{TcpListener, TcpStream}};
use std::collections::HashMap;
use std::io::{Read, Write};

use rkv::{KeyValueCmd};

fn main() {
    let mut db: HashMap<String, String> = HashMap::new();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_tcp_connection(stream, &mut db);
        }
    }
}

fn handle_tcp_connection(mut stream: TcpStream, db: &mut HashMap<String, String>) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let cmd_buffer = String::from_utf8_lossy(&buffer[..]);

    let kv_cmd = KeyValueCmd::new(&cmd_buffer);

    let buf = &kv_cmd.unwrap().run(db);
    let buf = &buf[..];
    stream.write(buf).unwrap();
}