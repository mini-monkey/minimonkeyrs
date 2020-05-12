mod minimonkey;
use std::net::TcpStream;
use std::time::Duration;

fn publish(stream: &mut TcpStream) {
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

    minimonkey::publish(stream, "hello".as_bytes()).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Publish failed {}", e),
        Ok(response) => response.print(),
    }
}

pub fn main() {
    match TcpStream::connect("localhost:1773") {
        Ok(mut stream) => {
            stream
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();
            publish(&mut stream);
        }
        Err(e) => {
            println!("Unable to publish: {}", e);
        }
    }
}
