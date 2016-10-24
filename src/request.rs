#[derive(Debug)]
pub struct Request {
    method: String,
    url: String,
    proto: String,
}

impl Request {
    pub fn new<S: Into<String>>(method: S, url: S, proto: S) -> Request {
        Request {
            method: method.into(),
            url: url.into(),
            proto: proto.into(),
        }
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}
