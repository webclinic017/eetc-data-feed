use std::thread;

extern crate zmq;

// Configuration
static BFX_DATA_FEED_SUB_ENDPOINT: &str = "tcp://localhost:5555";
static BFX_DATA_FEED_REQ_REP_ENDPOINT: &str = "tcp://localhost:5556";
static BFX_DATA_FEED_PUSH_PULL_ENDPOINT: &str = "ipc://bitfinex_data_feed";
static EETC_DATA_FEED_PUB_ENDPOINT: &str = "tcp://*:4444";
static EETC_DATA_FEED_ROUTER_ENDPOINT: &str = "tcp://*:4445";
static EETC_DATA_FEED_DEALER_ENDPOINT: &str = "ipc://bitfinex_hist_data_microservice";
static BFX_HIST_DATA_MICROSERVICE_THREADS: i16 = 2;

fn main() {
    // TODO implement logging
    println!("Started EETC Data Feed.");

    // ZeroMQ Context should be shared between treads but it doesn't implement Copy trait...
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

    // Spawn Thread for Bitfinex Hist Data Microservice
    thread::spawn(|| {
        println!("Started Bitfinex Hist Data Microservice Thread.");

        let zmq_context:zmq::Context = zmq::Context::new();

        let zmq_router_socket = zmq_context.socket(zmq::ROUTER).unwrap();
        let zmq_dealer_socket = zmq_context.socket(zmq::DEALER).unwrap();

        zmq_router_socket
            .bind(EETC_DATA_FEED_ROUTER_ENDPOINT)
            .expect("Could not bind ROUTER socket.");
        zmq_dealer_socket
            .bind(EETC_DATA_FEED_DEALER_ENDPOINT)
            .expect("Could not bind DEALER socket.");

        // Spawn Threads for Bitfinex Hist Data Microservice REQ-REP
        // TODO refaactor to spawn a worker thread for each request that comes in
        for i in 1..=BFX_HIST_DATA_MICROSERVICE_THREADS {
            thread::spawn(|| {
                let zmq_context:zmq::Context = zmq::Context::new();

                let zmq_rep_socket = zmq_context.socket(zmq::REP).unwrap();
                let zmq_req_socket = zmq_context.socket(zmq::REQ).unwrap();

                zmq_req_socket
                    .connect(BFX_DATA_FEED_REQ_REP_ENDPOINT)
                    .expect("Could not connect to REP");
                zmq_rep_socket
                    .connect(EETC_DATA_FEED_DEALER_ENDPOINT)
                    .expect("Could not connect to DEALER");

                loop {
                    // Receive request from client and forward it to the microservice
                    let request = zmq_rep_socket.recv_bytes(0).unwrap();
                    zmq_req_socket.send(request, 0).expect("ZMQ Error.");

                    // Receive response from the microservice and forward it to the client
                    let response = zmq_req_socket.recv_multipart(0).unwrap();
                    zmq_rep_socket.send_multipart(response, 0).expect("ZMQ Error.");
                }
            });
            println!("Started Bitfinex Hist Data Microservice Worker Thread {}.", i);
        }

        zmq::proxy(&zmq_router_socket, &zmq_dealer_socket).expect("ZMQ Proxy Error");

    });

    // Forward data from PULL Socket to the PUB Socket, to which the clients are subscribed to
    zmq::proxy(&zmq_pull_socket, &zmq_pub_socket).expect("ZMQ Proxy Error.");
}
