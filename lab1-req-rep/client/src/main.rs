#![crate_name = "client"]

fn main() {
    let context = zmq::Context::new();

    println!("Connecting to server...");
    let socket = context.socket(zmq::REQ).unwrap();

    assert!(socket.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();
    for request in 0..10 {
        // Send the request
        println!("Sending request {}...", request);
        socket.send("Hello", 0).unwrap();

        // Receive the reply
        socket.recv(&mut msg, 0).unwrap();
        println!("Received reply {}: {}", request, msg.as_str().unwrap());
    }
}
