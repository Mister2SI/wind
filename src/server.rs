use rsa::{RsaPrivateKey, RsaPublicKey};
use std::io::{prelude::*, ErrorKind};
use std::net::{TcpListener, TcpStream};

use crate::serializable_key::{self, SerializablePublicKey};

/// The entry point for the server
pub fn server(addr: &String) -> std::result::Result<(), ()> {
    // Get a stdout handle for flushing
    let mut stdout = std::io::stdout();

    // Attempt to open a listening socket
    print!("Opening listening socket... ");
    stdout.flush().unwrap();
    let listener;
    match TcpListener::bind(addr) {
        Ok(l) => listener = l,
        Err(e) => {
            match e.kind() {
                ErrorKind::InvalidInput => eprintln!("The given address is invalid."),
                _ => eprintln!("An unknown error occured while trying to open a listening socket."),
            }
            return Err(());
        }
    }
    println!("Done");

    // Generate an RSA keypair
    print!("Generating RSA keypair... ");
    stdout.flush().unwrap();
    let mut rng = rand::thread_rng();
    let prv_key;
    if let Ok(k) = RsaPrivateKey::new(&mut rng, 2048) {
        prv_key = k;
    } else {
        eprintln!("Failed to generate an RSA private key.");
        return Err(());
    }
    let pub_key = RsaPublicKey::from(&prv_key);
    println!("Done");
    let ser_key = SerializablePublicKey::from_rsa_key(&pub_key);
    let bin_key = bincode::serialize(&ser_key).unwrap();

    // Handle incoming connections
    println!("Accepting connections on \"{addr}\"...");
    stdout.flush().unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        print!(
            "New connection from \"{0}\"\nSending public encryption key... ",
            stream.peer_addr().unwrap()
        );
        stdout.flush().unwrap();
        if let Err(_) = stream.write_all(&bin_key) {
            eprintln!("An error occured while transmitting the public encryption key.");
            return Err(());
        }
        println!("Done");
    }

    Ok(())
}
