use rand::distributions::{Alphanumeric, DistString};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::net::UdpSocket;
use std::str;
use std::sync::mpsc::{self, RecvError};
use std::thread;

// MEMORY SEEMS TO GROW NOW, ODDLY

fn main() {
    // tuple of bytes up to 1024, followed by length of byte data
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        for received in rx {
            println!("Received {}", received);
        }
    });

    loop {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 128);
        tx.send(string).unwrap();
    }
}
