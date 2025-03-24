use rsa::{RsaPrivateKey, RsaPublicKey};
use std::io::ErrorKind;
use std::net::TcpStream;

/// The entry point for the client
pub fn client(addr: &String) -> std::result::Result<(), ()> {
    // Attempt to connect to the remote socket
    println!("Connecting...");
    let stream;
    match TcpStream::connect(addr) {
        Ok(s) => stream = s,
        Err(e) => {
            match e.kind() {
                ErrorKind::NetworkUnreachable => eprintln!("The network is unreachable."),
                ErrorKind::HostUnreachable => eprintln!("The specified host cannot be reached."),
                ErrorKind::AddrNotAvailable => {
                    eprintln!("The requested address was not available.")
                }
                ErrorKind::InvalidInput => eprintln!("The supplied address is not valid."),
                ErrorKind::ConnectionRefused => {
                    eprintln!("The connection was refused by the remote server.")
                }
                _ => {
                    eprintln!(
                        "An unknown error occured while trying to connect to the remote socket.\nError: {:#?}",
                        e
                    );
                }
            }
            return Err(());
        }
    }

    // Generate an RSA keypair
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

    // Return success
    Ok(())
}
