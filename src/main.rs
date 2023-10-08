use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8477";

const TEMP_WEB_PAGE: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Hello world</h1>
</body>
</html>
"#;

fn main() {
    let end_point: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(end_point);
    match listener {
        Ok(tcp_listener) => start_connection(tcp_listener),
        Err(err) => println!("Could not establish connection.\n{}", err),
    }
}

fn start_connection(listener: TcpListener) {
    println!("Web server is listening at port {}", PORT);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response_contents = get_response_contents();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_response_contents() -> String {
    match fs::read_to_string("index.html") {
        Ok(msg) => msg,
        Err(_) => TEMP_WEB_PAGE.to_owned(),
    } 
}
