use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REP).unwrap();

    assert!(socket.connect("tcp://localhost:5560").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        socket.recv(&mut msg, 0).unwrap();
        println!("Received request: {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000)); // visualize requests arriving
        socket.send("World", 0).unwrap();
    }
}
