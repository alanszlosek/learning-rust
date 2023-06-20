use std::io;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    // Create a simple streaming channel
    let (tx, rx) = channel();
    let thread_handle = thread::spawn(move|| {
        loop {
            let data = rx.recv().unwrap();
            println!("Got {}", data);
        }
    });

    let mut buffer = String::new();
    while buffer != "q" {
        println!("Type something: ");

        buffer = String::new();
        io::stdin().read_line(&mut buffer);
        tx.send(buffer.clone());
    }

    thread_handle.join();
}

