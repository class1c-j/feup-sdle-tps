#![crate_name = "server"]

use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REP).unwrap();

    assert!(socket.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();

    loop {
        // Receive the request
        socket.recv(&mut msg, 0).unwrap();
        println!("Received: {}", msg.as_str().unwrap());

        // Send the reply
        thread::sleep(Duration::from_millis(1000));
        socket.send("World", 0).unwrap();
    }
}
