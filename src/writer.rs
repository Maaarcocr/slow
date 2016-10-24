use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;

pub struct Writer {
    stream: TcpStream,
    headers: HashMap<String, String>,
    status_code: u16,
    description_code: String,
    body: String,
}

impl Writer {
    pub fn status<S: Into<String>>(&mut self, code: u16, description: S ) {
        self.status_code = code;
        self.description_code = description.into();
    }
    pub fn new(stream: TcpStream) -> Writer {
        Writer {
            stream: stream,
            headers: HashMap::new(),
            body: String::new(),
            status_code: 200,
            description_code: "OK".to_string(),
        }
    }
    pub fn write<S: Into<String>>(&mut self, body: S) {
        self.body = body.into();
    }
    pub fn add_header<S: Into<String>>(&mut self, name: S, value: S) {
        self.headers.insert(name.into(), value.into());
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let mut res = String::new();
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.description_code);
        res = res + &status_line;
        for (header_name, header_value) in self.headers.iter() {
            let partial = format!("{}: {}\r\n", header_name, header_value);
            res = res + &partial;
        }
        res = res + "\r\n";
        res = res + &self.body;
        println!("{:?}", res);
        self.stream.write(res.as_bytes());
    }
}
