use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let address = match env::var("PING_LISTEN_PORT") {
        Ok(val) => format!("0.0.0.0:{val}"),
        Err(_err) => format!("0.0.0.0:8080"),
    };
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Server listening : {:#?}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Spawn a new thread for each connection
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Retrieve the request from connection' stream
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("REQUEST : {}", request[0]);

    // Only if the requested URL is /ping and HTTP verb GET
    if request[0] == "GET /ping HTTP/1.1" || request[0] == "GET /ping HTTP/1.0" {
        // Forge the response body from the request's headers
        let mut response_body: Vec<String> = Vec::new();
        for data in request[1..].into_iter() {
            let (s1, s2) = data.split_once(": ").unwrap();
            response_body.push(format!("\"{}\": \"{}\"", s1, s2));
        }

        // Forge HTTP 200 response
        let response = format!("HTTP/1.1 200 OK\r\n");
        // Add Content-Type JSON header
        let response = format!("{response}Content-Type: application/json\r\n");
        // Add body
        let response = format!("{response}\r\n{{{}}}", response_body.join(","));
        // Send response
        match stream.write_all(response.as_bytes()) {
            Ok(_r) => (),
            // No panic
            Err(err) => println!("{err}"),
        }
    } else {
        // Forge HTTP 404 response
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n");
        match stream.write_all(response.as_bytes()) {
            Ok(_r) => (),
            // No panic
            Err(err) => println!("{err}"),
        }
    }
}
