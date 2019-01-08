use std::thread;

extern crate zmq;

static BFX_DATA_FEED_SUB_ENDPOINT: &str = "tcp://localhost:5555";
static BFX_DATA_FEED_REQ_REP_ENDPOINT: &str = "tcp://localhost:5556";
static EETC_DATA_FEED_PUSH_PULL_ENDPOINT: &str = "ipc://eetc_data_feed";
static EETC_DATA_FEED_PUB_ENDPOINT: &str = "tcp://*:4444";
static EETC_DATA_FEED_REQ_REP_ENDPOINT_BFX: &str = "tcp://*:4445";
static EETC_DATA_FEED_DEALER_ENDPOINT: &str = "ipc://bitfinex_hist_data_microservice";
static BFX_HIST_DATA_MICROSERVICE_THREADS: i16 = 2;

fn main() {
    // TODO implement logging
    println!("Started EETC Data Feed.");

    let zmq_context:zmq::Context = zmq::Context::new();

    let zmq_pub_socket = zmq_context.socket(zmq::PUB).unwrap();
    let zmq_pull_socket = zmq_context.socket(zmq::PULL).unwrap();

    zmq_pull_socket
        .bind(EETC_DATA_FEED_PUSH_PULL_ENDPOINT)
        .expect("Could not bind PULL socket.");
    zmq_pub_socket
        .bind(EETC_DATA_FEED_PUB_ENDPOINT)
        .expect("Could not bind PUB socket.");

    // Spawn Thread for Bitfinex Data Feed PUB-SUB
    thread::spawn(move || {
        println!("Started Bitfinex Data Feed PUB-SUB Thread.");
        bitfinex_data_feed_pub_sub_routine(&zmq_context);
    });

    // Spawn Thread for REQ-REP for Bitfinex Hist Data Microservice
    thread::spawn(|| {
        println!("Started Bitfinex Data Feed REQ-REP Thread.");
        bitfinex_data_feed_req_rep_routine();
    });

    // TODO Spawn Thread for InteractiveBrokers Data Feed PUB-SUB

    // TODO Spawn Thread for InteractiveBrokers Data Feed REQ-REP

    zmq::proxy(&zmq_pull_socket, &zmq_pub_socket).expect("ZMQ Proxy Error.");
}

fn bitfinex_data_feed_pub_sub_routine(zmq_context:&zmq::Context) {
    let zmq_sub_socket = zmq_context.socket(zmq::SUB).unwrap();
    let zmq_push_socket = zmq_context.socket(zmq::PUSH).unwrap();

    zmq_push_socket
        .connect(EETC_DATA_FEED_PUSH_PULL_ENDPOINT)
        .expect("Could not connect to PULL");
    zmq_sub_socket
        .connect(BFX_DATA_FEED_SUB_ENDPOINT)
        .expect("Could not connect to PUB.");

    zmq_sub_socket
        .set_subscribe(b"") //  "" is the default value to subscribe to all topics
        .expect("Could not subscribe to topic.");

    zmq::proxy(&zmq_sub_socket, &zmq_push_socket).expect("ZMQ Proxy Error.");
}

fn bitfinex_data_feed_req_rep_routine() {
    let zmq_context:zmq::Context = zmq::Context::new();

    let zmq_router_socket = zmq_context.socket(zmq::ROUTER).unwrap();
    let zmq_dealer_socket = zmq_context.socket(zmq::DEALER).unwrap();

    zmq_router_socket
        .bind(EETC_DATA_FEED_REQ_REP_ENDPOINT_BFX)
        .expect("Could not bind ROUTER socket.");
    zmq_dealer_socket
        .bind(EETC_DATA_FEED_DEALER_ENDPOINT)
        .expect("Could not bind DEALER socket.");

    // Spawn Worker Threads
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

            forward_request(&zmq_rep_socket, &zmq_req_socket);
        });

        println!("Started Bitfinex Hist Data Microservice Worker Thread {}.", i);
    }

    zmq::proxy(&zmq_router_socket, &zmq_dealer_socket).expect("ZMQ Proxy Error");
}

fn forward_request(zmq_rep_socket: &zmq::Socket, zmq_req_socket: &zmq::Socket) {
    loop {
        let request = zmq_rep_socket.recv_bytes(0).unwrap();
        zmq_req_socket.send(request, 0).expect("ZMQ Error.");

        let response = zmq_req_socket.recv_multipart(0).unwrap();
        zmq_rep_socket.send_multipart(response, 0).expect("ZMQ Error.");
    }
}
