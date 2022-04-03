use std::io::Write;

#[derive(Default)]
struct HttpHeader {
    version: &'static str,
    content_type: &'static str,
    message: &'static str,
}

impl HttpHeader {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn set_version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    fn set_content_type(mut self, content_type: &'static str) -> Self {
        self.content_type = content_type;
        self
    }

    fn set_message(mut self, message: &'static str) -> Self {
        self.message = message;
        self
    }

    fn build(&self) -> String {
        format!("{}{}{}", self.version, self.content_type, self.message)
    }
}
fn main() {
    let response = HttpHeader::new();
    let page = response
        .set_version("HTTP/1.1 200 OK\r\n")
        .set_content_type("Content-Type: text/html\r\n\r\n")
        .set_message("<html><header><title>Application</title></header><body><h1>Hello, world!</h1></body></html>")
        .build();
    let server = std::net::TcpListener::bind("127.0.0.1:5000").unwrap();
    println!("Listen on: 127.0.0.1:5000");
    loop {
        match server.accept() {
            Ok((mut socket, _address)) => {
                socket.write(page.as_bytes()).unwrap();
            },
            Err(_err) => {}
        }
    }
}
