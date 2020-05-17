use clap::{App, Arg};
use minimonkey;
use std::io::Error;
use std::net::TcpStream;
use std::process;
use std::time::Duration;

struct TaggedMessage {
    pub tag: String,
    pub msg: String,
}

fn subscribe_common(stream: &mut TcpStream, token: &str, room: &str, tag: &str) {
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
}

fn receive_tag_and_message(stream: &mut TcpStream) -> Result<TaggedMessage, Error> {
    let tag_response = minimonkey::read(stream)?;
    let msg_response = minimonkey::read(stream)?;

    Ok(TaggedMessage {
        tag: tag_response.as_string(),
        msg: msg_response.as_string(),
    })
}

fn subscribe(stream: &mut TcpStream, token: &str, room: &str, tag: &str) {
    subscribe_common(stream, token, room, tag);

    loop {
        match receive_tag_and_message(stream) {
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
            Ok(tagged_response) => {
                let tag = tagged_response.tag;
                let msg = tagged_response.msg;
                println!("[{}] {}", tag, msg)
            }
        }
        Duration::from_secs(1);
    }
}

fn subscribe_and_assert_string(
    stream: &mut TcpStream,
    token: &str,
    room: &str,
    tag: &str,
    assert_msg: &str,
    _assert_timeout: &str,
) {
    subscribe_common(stream, token, room, tag);
    match receive_tag_and_message(stream) {
        Err(_) => {
            process::exit(2);
        }
        Ok(tagged_response) => {
            if tagged_response.tag != tag {
                process::exit(3);
            }
            if tagged_response.msg != assert_msg {
                process::exit(4);
            }
            process::exit(0);
        }
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
        .arg(
            Arg::with_name("assert")
                .short("a")
                .long("assert")
                .help("assert against string and exit"),
        )
        .arg(
            Arg::with_name("assert-tag")
                .long("assert-tag")
                .takes_value(true)
                .default_value("")
                .help("tag to assert against"),
        )
        .arg(
            Arg::with_name("assert-msg")
                .long("assert-msg")
                .takes_value(true)
                .help("message to assert against"),
        )
        .arg(
            Arg::with_name("assert-timeout")
                .long("assert-timeout")
                .takes_value(true)
                .help("timeout when asserting string"),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("1773");
    let token = matches.value_of("token").unwrap_or("myToken");
    let room = matches.value_of("room").unwrap_or("myRoom");
    let tag = matches.value_of("tag").unwrap_or("myTag");
    let assert = matches.is_present("assert");

    match TcpStream::connect(format!("{}:{}", host, port)) {
        Err(e) => {
            println!("Unable to subscribe: {}", e);
        }
        Ok(mut stream) => {
            if assert {
                let assert_msg = matches.value_of("assert-msg").unwrap_or("");
                let assert_timeout = matches.value_of("assert_timeout").unwrap_or("30000");
                subscribe_and_assert_string(
                    &mut stream,
                    token,
                    room,
                    tag,
                    assert_msg,
                    assert_timeout,
                );
            } else {
                subscribe(&mut stream, token, room, tag)
            }
        }
    }
}
