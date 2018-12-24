use std::thread;

extern crate zmq;

// Configuration
static BFX_DATA_FEED_SUB_ENDPOINT: &str = "tcp://localhost:5555";
static BFX_DATA_FEED_PUSH_PULL_ENDPOINT: &str = "ipc://bitfinex_data_feed";
static EETC_DATA_FEED_PUB_ENDPOINT: &str = "tcp://*:4444";

fn main() {
    // TODO implement logging
    println!("Started EETC Data Feed.");

    // Create ZeroMQ Context which will be shared between threads
    let zmq_context:zmq::Context = zmq::Context::new();

    // Create ZeroMQ Sockets that we'll use in the main thread
    let zmq_pub_socket = zmq_context.socket(zmq::PUB).unwrap();
    let zmq_pull_socket = zmq_context.socket(zmq::PULL).unwrap();

    // Bind Sockets to endpoints
    zmq_pull_socket
        .bind(BFX_DATA_FEED_PUSH_PULL_ENDPOINT)
        .expect("Could not bind PULL socket.");
    zmq_pub_socket
        .bind(EETC_DATA_FEED_PUB_ENDPOINT)
        .expect("Could not bind PUB socket.");

    // Spawn Thread for Bitfinex Data Feed
    thread::spawn(move || {
        println!("Started Bitfinex Data Feed PUB-SUB Thread.");

        // Create ZeroMQ Sockets that we'll use in this thread & use zmq_context from main thread
        let zmq_sub_socket = zmq_context.socket(zmq::SUB).unwrap();
        let zmq_push_socket = zmq_context.socket(zmq::PUSH).unwrap();

        // Connect Sockets to endpoints
        zmq_push_socket
            .connect(BFX_DATA_FEED_PUSH_PULL_ENDPOINT)
            .expect("Could not connect to PULL");
        zmq_sub_socket
            .connect(BFX_DATA_FEED_SUB_ENDPOINT)
            .expect("Could not connect to PUB.");

        // Set subscription topic for SUB Socket
        zmq_sub_socket
            .set_subscribe(b"") //  "" is the default value to subscribe to all topics
            .expect("Could not subscribe to topic.");

        // Forward data from SUB Socket to PUSH Socket, which sends it to the PULL Socket in the main thread
        zmq::proxy(&zmq_sub_socket, &zmq_push_socket).expect("ZMQ Proxy Error.");
    });

    // Forward data from PULL Socket to the PUB Socket, to which the clients are subscribed to
    zmq::proxy(&zmq_pull_socket, &zmq_pub_socket).expect("ZMQ Proxy Error.");
}
