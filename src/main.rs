
mod rkv;

use std::{net::{TcpListener, TcpStream}};
use std::collections::HashMap;
use std::io::{Read, Write};

use rkv::{KeyValueCmd, KeyValueAction};

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

    if let Result::Ok(kv_cmd) = kv_cmd {
        match kv_cmd.action {
            KeyValueAction::KvSet => {
                let buf = format!("{}: {}\n",kv_cmd.key, kv_cmd.value);
                db.insert(kv_cmd.key, kv_cmd.value);
                stream.write(buf.as_bytes()).unwrap();
            },
            KeyValueAction::KvGet => {
                let buf = db.get(&kv_cmd.key);
                match buf {
                    Some(buf) => {stream.write(buf.as_bytes()).unwrap();},
                    None => {stream.write("Cannot found!!!\n".as_bytes()).unwrap();}
                }
            }
            KeyValueAction::KvDel => {
                let buf = db.remove(&kv_cmd.key);
                match buf {
                    Some(_buf) => {stream.write("Removed.\n".as_bytes()).unwrap();}
                    None => {stream.write("Cannot found!!!\n".as_bytes()).unwrap();}
                }
            }
            _ => {
                stream.write("Unsupport!\n".as_bytes()).unwrap();
            }
        }
    }
}