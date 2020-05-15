mod minimonkey;
use clap::{App, Arg};
use std::net::TcpStream;
use std::time::Duration;

fn publish(stream: &mut TcpStream, token: &str, room: &str, message: &str) {
    minimonkey::authenticate(stream, token).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Authentication failed {}", e),
        Ok(response) => response.print(),
    }

    minimonkey::enter(stream, room).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Enter failed {}", e),
        Ok(response) => response.print(),
    }

    let data = message.as_bytes();
    minimonkey::publish(stream, data).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Publish failed {}", e),
        Ok(response) => response.print(),
    }
}

pub fn main() {
    let matches = App::new("mm_publish")
        .version("0.1.0")
        .author("Niklas Johansson <raphexion@gmail.com>")
        .about("Publish message to Mini Monkey broker")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .takes_value(true)
                .help("hostname of mini monkey broker"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("port of the mini monkey broker"),
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .takes_value(true)
                .default_value("myToken")
                .help("token for authentication"),
        )
        .arg(
            Arg::with_name("room")
                .short("r")
                .long("room")
                .takes_value(true)
                .help("room to publish message"),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .alias("msg")
                .takes_value(true)
                .help("message to publish in room"),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("1773");
    let token = matches.value_of("token").unwrap_or("myToken");
    let room = matches.value_of("room").unwrap_or("myRoom");
    let message = matches.value_of("message").unwrap_or("myMessage");

    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(mut stream) => {
            stream
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();
            publish(&mut stream, token, room, message);
        }
        Err(e) => {
            println!("Unable to publish: {}", e);
        }
    }
}
