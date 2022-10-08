fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REQ).unwrap();

    assert!(socket.connect("tcp://localhost:5559").is_ok());

    let mut msg = zmq::Message::new();

    // Do 10 requests, waiting each time for a response
    for request in 1..10 {
        socket.send("Hello", 0).unwrap();
        socket.recv(&mut msg, 0).unwrap();
        println!("Received reply {} [{}]", request, msg.as_str().unwrap());
    }
}
