use std::env;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB).unwrap();

    println!("Collecting updates from weather server");
    assert!(socket.connect("tcp://localhost:5556").is_ok());

    // Subscribe to zipcode, default is NYC 10001, but can be passed on args
    let args: Vec<String> = env::args().collect();
    let zip_filter = if args.len() > 1 { &args[1] } else { "10001" };

    assert!(socket.set_subscribe(zip_filter.as_bytes()).is_ok());
    // will not match exactly... 1001 will match to 10012, fix?

    let mut total_temp = 0;

    for update_number in 0..5 {
        let data = socket.recv_string(0).unwrap().unwrap();
        let chunks: Vec<i64> = data
            .split(' ')
            .map(|s| -> i64 { s.parse().unwrap() })
            .collect();
        let temperature = chunks[1];

        total_temp += temperature;
        println!(
            "Average temperature for zipcode {} was {} F",
            zip_filter,
            total_temp / (update_number + 1)
        );
    }
}
