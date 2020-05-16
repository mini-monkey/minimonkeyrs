use clap::{App, Arg};
use minimonkey;
use std::net::TcpStream;
use std::time::Duration;

fn subscribe(stream: &mut TcpStream, token: &str, room: &str, tag: &str) {
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

    minimonkey::subscribe(stream, tag).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Subscribe failed {}", e),
        Ok(response) => response.print(),
    }

    loop {
        match minimonkey::read(stream) {
            Err(e) => println!("Read failed {}", e),
            Ok(tag_response) => {
                let tag = tag_response.as_string();
                match minimonkey::read(stream) {
                    Err(e) => println!("Failed to read message {}", e),
                    Ok(message_response) => {
                        let msg = message_response.as_string();
                        println!("[{}] {}", tag, msg);
                    }
                };
            }
        }
        Duration::from_secs(1);
    }
}

pub fn main() {
    let matches = App::new("mm_subscribe")
        .version("0.1.2")
        .author("Niklas Johansson <raphexion@gmail.com>")
        .about("Subscribe to messages on Mini Monkey broker")
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
            Arg::with_name("tag")
                .long("tag")
                .takes_value(true)
                .help("tag for subscription"),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("1773");
    let token = matches.value_of("token").unwrap_or("myToken");
    let room = matches.value_of("room").unwrap_or("myRoom");
    let tag = matches.value_of("tag").unwrap_or("");

    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(mut stream) => subscribe(&mut stream, token, room, tag),
        Err(e) => {
            println!("Unable to subscribe: {}", e);
        }
    }
}
