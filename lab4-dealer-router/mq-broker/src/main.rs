fn main() {
    let context = zmq::Context::new();

    let frontend_socket = context.socket(zmq::ROUTER).unwrap();
    let backend_socket = context.socket(zmq::DEALER).unwrap();

    assert!(frontend_socket.bind("tcp://*:5559").is_ok());
    assert!(backend_socket.bind("tcp://*:5560").is_ok());

    zmq::proxy(&frontend_socket, &backend_socket).unwrap();
}
