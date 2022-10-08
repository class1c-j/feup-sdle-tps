fn main() {
    let context = zmq::Context::new();
    let frontend_socket = context.socket(zmq::ROUTER).unwrap();
    let backend_socket = context.socket(zmq::DEALER).unwrap();

    assert!(frontend_socket.bind("tcp://*:5559").is_ok());
    assert!(backend_socket.bind("tcp://*:5560").is_ok());

    loop {
        let mut items = [
            frontend_socket.as_poll_item(zmq::POLLIN),
            backend_socket.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            loop {
                let msg = frontend_socket.recv_msg(0).unwrap();
                let more = msg.get_more();
                backend_socket
                    .send(msg, if more { zmq::SNDMORE } else { 0 })
                    .unwrap();
                if !more {
                    break;
                }
            }
        }

        if items[1].is_readable() {
            loop {
                let msg = backend_socket.recv_msg(0).unwrap();
                let more = msg.get_more();
                frontend_socket
                    .send(msg, if more { zmq::SNDMORE } else { 0 })
                    .unwrap();
                if !more {
                    break;
                }
            }
        }
    }
}
