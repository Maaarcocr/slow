use writer::*;
use std::thread;
use regex::Regex;
use std::net::SocketAddr;
use request::*;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;


#[derive(Clone)]
pub struct Slow {
    urls: HashMap<String, fn(Request, Writer)>
}

impl Slow {
    pub fn new() -> Slow {
        Slow {
            urls: HashMap::new()
        }
    }
    pub fn start(&self, port: &str) {
        let address: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        let listener = TcpListener::bind(address).unwrap();
        println!("Connected");
        //let arcSelf = Arc::new(Mutex::new(self));
        // accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
            println!("1 Request");
            match stream {
                Ok(stream) => {
                    let self_clone = self.clone();
                    println!("Ok");
                    thread::spawn(move|| {
                        // connection succeeded
                        self_clone.handler_filter(stream)
                        //self_clone.handler_filter(stream)
                    });
                }
                Err(e) => panic!(e)
            }
        }
        // close the socket server
        drop(listener);
    }
    pub fn add_handler<S: Into<String>>(&mut self, url: S, handler: fn(Request, Writer)) {
        self.urls.insert(url.into(), handler);
    }
    fn handler_filter(&self, stream: TcpStream) {
        let (stream, stream_str) = read_stream(stream);
        let first_line = stream_str.lines().nth(0).unwrap();
        let request = create_request_object(&String::from(first_line)).unwrap();
        match self.urls.get(&request.get_url()) {
            Some(f) => f(request, Writer::new(stream)),
            None => no_match(Writer::new(stream)),
        }
    }
}

fn create_request_object(s: &String) -> Option<Request> {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"([^\s]*)\s([^\s]*)\s([^\s]*)").unwrap();
    }
    let cap = {
        match RE.captures(s) {
            Some(c) => c,
            None => return None,
        }
    };
    let method = cap.at(1).unwrap();
    let url = cap.at(2).unwrap();
    let proto = cap.at(3).unwrap();
    Some(
        Request::new(method, url, proto)
    )
}

fn read_stream(mut stream: TcpStream) -> (TcpStream, String) {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).unwrap();
    let mut text = String::new();
    for b in buf.iter() {
        if *b == 0 {
            break
        }
        text.push(*b as char)
    }
    (stream, text)
}

fn no_match(mut writer: Writer){
    writer.status(400, "NOT FOUND")
}
