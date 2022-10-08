use std::env;

fn main() {
    let context = zmq::Context::new();

    let us_socket = context.socket(zmq::SUB).unwrap();
    let pt_socket = context.socket(zmq::SUB).unwrap();

    assert!(us_socket.connect("tcp://localhost:5556").is_ok());
    assert!(pt_socket.connect("tcp://localhost:5557").is_ok());

    let args: Vec<String> = env::args().collect();
    let us_zip_filter = if args.len() > 1 { &args[1] } else { "10001" };
    let pt_zip_filter = if args.len() > 2 { &args[2] } else { "5100" };

    assert!(us_socket.set_subscribe(us_zip_filter.as_bytes()).is_ok());
    assert!(pt_socket.set_subscribe(pt_zip_filter.as_bytes()).is_ok());

    let mut msg = zmq::Message::new();
    let mut us_count = 0;
    let mut us_total = 0;
    let mut pt_count = 0;
    let mut pt_total = 0;

    loop {
        let mut items = [
            us_socket.as_poll_item(zmq::POLLIN),
            pt_socket.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();
        if us_count < 5 && items[0].is_readable() && us_socket.recv(&mut msg, 0).is_ok() {
            // received US update
            let data = msg.as_str().unwrap();
            println!("[FROM US SERVER] {}", data);
            let chunks: Vec<i64> = data
                .split(' ')
                .map(|s| -> i64 { s.parse().unwrap() })
                .collect();
            us_count += 1;
            us_total += chunks[1];
        }
        if pt_count < 5 && items[1].is_readable() && pt_socket.recv(&mut msg, 0).is_ok() {
            // received PT update
            let data = msg.as_str().unwrap();
            println!("[FROM PT SERVER] {}", data);
            let chunks: Vec<i64> = data
                .split(' ')
                .map(|s| -> i64 { s.parse().unwrap() })
                .collect();
            pt_count += 1;
            pt_total += chunks[1];
        }
        if pt_count == 5 && us_count == 5 {
            println!(
                "Average temperature US was {} and PT was {}",
                us_total / 5,
                pt_total / 5
            );
            break;
        }
    }
}
