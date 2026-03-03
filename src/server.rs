use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::store::Store;

pub fn run(addr: &str) -> std::io::Result<()> {
    let listener: TcpListener = match TcpListener::bind(addr) {
        Ok(tcp) => tcp,
        Err(e) => panic!("Unable to bind to {addr}, Error: {e}"),
    };
    let concurrent_store = Arc::new(Mutex::new(Store::new()));
    for data in listener.incoming() {
        let mut stream: TcpStream = match data {
            Ok(tcp_stream) => tcp_stream,
            Err(e) => {
                eprint!("Error :{e}");
                break;
            }
        };
        let store = Arc::clone(&concurrent_store);
        std::thread::spawn(move || {
            handle_connection(&mut stream, store);
        });
    }
    Ok(())
}

fn handle_connection(stream: &mut TcpStream, store: Arc<Mutex<Store>>) {
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        let length = match stream.read(&mut buffer) {
            Ok(len) => len,
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        };
        if length == 0 {
            break;
        }
        let response = String::from_utf8_lossy(&buffer[..length]);
        for line in response.lines() {
            let response = handle_command(line, &store);
            #[allow(unused)]
            stream.write_all(response.as_bytes());
            #[allow(unused)]
            stream.write_all("\n".as_bytes());
        }
    }
}

fn handle_command(line: &str, store: &Arc<Mutex<Store>>) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut iterator = words.iter();
    let command = match iterator.next() {
        Some(word) => *word,
        None => return String::from("INVALID SYNTAX"),
    };
    return {
        let mut map_guard = store.lock().unwrap();
        match command {
            "get" => match iterator.next() {
                Some(value) => map_guard.get(*value),
                None => String::from("KEY ARG MISSING!"),
            },
            "set" => match iterator.next() {
                Some(arg1) => match iterator.next() {
                    Some(arg2) => {
                        let mut second_arg = String::from(*arg2);
                        for word in iterator.by_ref() {
                            second_arg.push_str(" ");
                            second_arg.push_str(word);
                        }
                        map_guard.set(*arg1, second_arg)
                    }
                    None => String::from("VALUE ARG MISSING!"),
                },
                None => String::from("KEY ARG"),
            },
            "delete" => match iterator.next() {
                Some(value) => map_guard.delete(*value),
                None => String::from("KEY ARG MISSING!"),
            },
            _ => String::from("INVALID SYNTAX"),
        }
    };
}
