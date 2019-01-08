pub mod constants{
    pub static BFX_DATA_FEED_SUB_ENDPOINT: &str = "tcp://localhost:5555";
    pub static BFX_DATA_FEED_REQ_REP_ENDPOINT: &str = "tcp://localhost:5556";
    pub static EETC_DATA_FEED_PUSH_PULL_ENDPOINT: &str = "ipc://eetc_data_feed";
    pub static EETC_DATA_FEED_PUB_ENDPOINT: &str = "tcp://*:4444";
    pub static EETC_DATA_FEED_REQ_REP_ENDPOINT_BFX: &str = "tcp://*:4445";
    pub static EETC_DATA_FEED_DEALER_ENDPOINT: &str = "ipc://bitfinex_hist_data_microservice";
    pub static BFX_HIST_DATA_MICROSERVICE_THREADS: i16 = 2;
}