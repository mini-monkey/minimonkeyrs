use clap::{App, Arg};
use minimonkey::{add_admin_permission, revoke_admin_permission};
use minimonkey::{add_login_permission, revoke_login_permission};
use minimonkey::{add_publish_permission, revoke_publish_permission};
use minimonkey::{add_subscribe_permission, revoke_subscribe_permission};
use minimonkey::{ProvisionInfo, Room};
use serde_yaml;
use std::fs::File;
use std::io::Error;
use std::net::TcpStream;
use std::time::Duration;
mod utils;

use utils::BrokerAccess;

type TokenOperation = fn(&mut TcpStream, &str) -> Result<usize, Error>;

fn provision_tokens(
    stream: &mut TcpStream,
    tokens: &Option<Vec<String>>,
    op: TokenOperation,
) -> Result<(), Error> {
    if let Some(tokens) = tokens {
        for token in tokens {
            op(stream, token)?;
        }
    }
    Ok(())
}

fn provision_room(stream: &mut TcpStream, room: &Room) -> Result<BrokerAccess, Error> {
    minimonkey::enter(stream, &room.name).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Enter failed {}", e),
        Ok(response) => response.print(),
    }

    let mut broker_access = BrokerAccess::new();

    // Add and revoke admin permissions
    provision_tokens(stream, &room.tokens_allowed_to_admin, add_admin_permission)?;
    provision_tokens(
        stream,
        &room.tokens_disallowed_to_admin,
        revoke_admin_permission,
    )?;
    broker_access.mark_access(&room.tokens_allowed_to_admin);
    broker_access.mark_revoked(&room.tokens_disallowed_to_admin);

    // Add and revoke publish permissions
    provision_tokens(
        stream,
        &room.tokens_allowed_to_publish,
        add_publish_permission,
    )?;
    provision_tokens(
        stream,
        &room.tokens_disallowed_to_publish,
        revoke_publish_permission,
    )?;
    broker_access.mark_access(&room.tokens_allowed_to_publish);
    broker_access.mark_revoked(&room.tokens_disallowed_to_publish);

    // Add and revoke subscribe permissions
    provision_tokens(
        stream,
        &room.tokens_allowed_to_subscribe,
        add_subscribe_permission,
    )?;
    provision_tokens(
        stream,
        &room.tokens_disallowed_to_subscribe,
        revoke_subscribe_permission,
    )?;
    broker_access.mark_access(&room.tokens_allowed_to_subscribe);
    broker_access.mark_revoked(&room.tokens_disallowed_to_subscribe);

    Ok(broker_access)
}

fn provision_rooms(stream: &mut TcpStream, info: &ProvisionInfo) -> Result<BrokerAccess, Error> {
    let mut broker_access = BrokerAccess::new();
    for room in &info.rooms {
        let room_broker_access = provision_room(stream, &room)?;
        broker_access.extend(&room_broker_access);
    }
    Ok(broker_access)
}

fn provision_broker_access(
    stream: &mut TcpStream,
    broker_access: &BrokerAccess,
) -> Result<(), Error> {
    let needs_login_access: Vec<String> =
        broker_access.needs_revoked_access().into_iter().collect();

    let needs_revoked_access: Vec<String> =
        broker_access.needs_revoked_access().into_iter().collect();

    let needs_login_access = Some(needs_login_access);
    let needs_revoked_access = Some(needs_revoked_access);

    provision_tokens(stream, &needs_login_access, add_login_permission)?;
    provision_tokens(stream, &needs_revoked_access, revoke_login_permission)?;

    Ok(())
}

fn provision(stream: &mut TcpStream, token: &str, file: &str) -> Result<(), Error> {
    minimonkey::authenticate(stream, token).unwrap();
    match minimonkey::read(stream) {
        Err(e) => println!("Authentication failed {}", e),
        Ok(response) => response.print(),
    }

    let file = File::open(&file).expect("Unable to open provision file");
    let info: ProvisionInfo = serde_yaml::from_reader(file).unwrap();
    let broker_access = provision_rooms(stream, &info)?;
    provision_broker_access(stream, &broker_access)?;

    Ok(())
}

fn main() {
    let matches = App::new("mm_provision")
        .version("0.1.2")
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
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("yaml file with information about provision"),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("1773");
    let token = matches.value_of("token").unwrap_or("myToken");
    let file = matches.value_of("file").unwrap();

    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(mut stream) => {
            stream
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();
            match provision(&mut stream, token, file) {
                Err(e) => println!("Provision failed {}", e),
                Ok(_) => println!("Provision success"),
            }
        }
        Err(e) => {
            println!("Unable to provision: {}", e);
        }
    }
}
