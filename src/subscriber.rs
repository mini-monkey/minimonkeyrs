mod minimonkey;
use std::net::TcpStream;
use std::time::Duration;

fn subscribe(stream: &mut TcpStream) {
    minimonkey::authenticate(stream, "myToken").unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Authentication failed {}", e),
        Ok(response) => response.print(),
    }

    minimonkey::enter(stream, "temperatures").unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Enter failed {}", e),
        Ok(response) => response.print(),
    }

    minimonkey::subscribe(stream, "subTag").unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Subscribe failed {}", e),
        Ok(response) => response.print(),
    }

    loop {
        match minimonkey::read(stream) {
            Err(e) => println!("Read failed {}", e),
            Ok(response) => response.print(),
        }
        Duration::from_secs(1);
    }
}

pub fn main() {
    match TcpStream::connect("localhost:1773") {
        Ok(mut stream) => subscribe(&mut stream),
        Err(e) => {
            println!("Unable to publish: {}", e);
        }
    }
}
