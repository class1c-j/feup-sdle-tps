use rand::{self, Rng};

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB).unwrap();
    assert!(socket.bind("tcp://*:5556").is_ok());

    let mut rng = rand::thread_rng();

    loop {
        let zipcode = rng.gen_range(1..100000);
        let temperature = rng.gen_range(-80..135);
        let relhumidity = rng.gen_range(10..60);
        let data = format!("{} {} {}", zipcode, temperature, relhumidity);
        socket.send(&data, 0).unwrap();
    }
}
