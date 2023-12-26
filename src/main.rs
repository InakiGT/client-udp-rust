use std::net::TcpStream;
use std::io::{self, Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:12345").expect("Error connecting with the server");

    println!("Server connected");

    loop {
        let mut input = String::new();
        print!("Ingress the message:");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error in the prompt");

        stream.write_all(input.as_bytes()).expect("Error in message sending");

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("{}", message);
            },
            Err(err) => {
                eprintln!("Error with the server data: {}", err)
            }
        }
    }   
}
