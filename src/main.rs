use std::thread;

extern crate zmq;

mod conf;

fn main() {
    // TODO implement logging
    println!("Started EETC Data Feed.");

    let zmq_context:zmq::Context = zmq::Context::new();

    let zmq_xpub_socket = zmq_context.socket(zmq::XPUB).unwrap();
    let zmq_xsub_socket = zmq_context.socket(zmq::XSUB).unwrap();

    zmq_xpub_socket
        .bind(conf::constants::EETC_DATA_FEED_PUB_ENDPOINT)
        .expect("Could not bind PUB socket.");
    zmq_xsub_socket
        .connect(conf::constants::BFX_DATA_FEED_SUB_ENDPOINT)
        .expect("Could not connect to PUB.");

    // Spawn Thread for REQ-REP for Bitfinex Hist Data Microservice
    thread::spawn(|| {
        println!("Started Bitfinex Data Feed REQ-REP Thread.");
        bitfinex_data_feed_req_rep_routine();
    });

    zmq::proxy(&zmq_xsub_socket, &zmq_xpub_socket).expect("ZMQ Proxy Error.");
}

fn bitfinex_data_feed_req_rep_routine() {
    let zmq_context:zmq::Context = zmq::Context::new();

    let zmq_router_socket = zmq_context.socket(zmq::ROUTER).unwrap();
    let zmq_dealer_socket = zmq_context.socket(zmq::DEALER).unwrap();

    zmq_router_socket
        .bind(conf::constants::EETC_DATA_FEED_REQ_REP_ENDPOINT_BFX)
        .expect("Could not bind ROUTER socket.");
    zmq_dealer_socket
        .bind(conf::constants::EETC_DATA_FEED_DEALER_ENDPOINT)
        .expect("Could not bind DEALER socket.");

    // Spawn Worker Threads
    for i in 1..=conf::constants::BFX_HIST_DATA_MICROSERVICE_THREADS {
        thread::spawn(|| {
            let zmq_context:zmq::Context = zmq::Context::new();

            let zmq_rep_socket = zmq_context.socket(zmq::REP).unwrap();
            let zmq_req_socket = zmq_context.socket(zmq::REQ).unwrap();

            zmq_req_socket
                .connect(conf::constants::BFX_DATA_FEED_REQ_REP_ENDPOINT)
                .expect("Could not connect to REP");
            zmq_rep_socket
                .connect(conf::constants::EETC_DATA_FEED_DEALER_ENDPOINT)
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
