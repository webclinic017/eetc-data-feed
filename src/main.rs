extern crate zmq;

fn main() {
    // TODO refactor this program to be multithreaded for each microservice it's subbed to
    // TODO implement logging
    println!("Started EETC Data Feed.");

    let zmq_context:zmq::Context = zmq::Context::new();

    let zmq_sub_socket = zmq_context.socket(zmq::SUB).unwrap();
    zmq_sub_socket
        .connect("tcp://localhost:5555")
        .expect("Could not connect to PUB.");
    zmq_sub_socket
        .set_subscribe(b"")
        .expect("Could not subscribe to topic.");

    let zmq_pub_socket = zmq_context.socket(zmq::PUB).unwrap();
    zmq_pub_socket
        .bind("tcp://*:4444")
        .expect("Could not bind PUB socket.");

    zmq::proxy(&zmq_sub_socket, &zmq_pub_socket).expect("ZMQ Proxy Error.");
}
