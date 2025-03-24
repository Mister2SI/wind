use rsa::{RsaPrivateKey, RsaPublicKey};
use std::io::{prelude::*, ErrorKind};
use std::net::{TcpListener, TcpStream};

/// The entry point for the server
pub fn server(addr: &String) -> std::result::Result<(), ()> {
    // Attempt to open a listening socket
    println!("Opening listening socket...");
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

    // Generate and RSA keypair
    println!("Generating RSA keypair...");
    let mut rng = rand::thread_rng();
    let prv_key;
    if let Ok(k) = RsaPrivateKey::new(&mut rng, 2048) {
        prv_key = k;
    } else {
        eprintln!("Failed to generate an RSA private key.");
        return Err(());
    }
    let pub_key = RsaPublicKey::from(&prv_key);

    Ok(())
}
