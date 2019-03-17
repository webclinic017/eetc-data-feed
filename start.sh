#!/usr/bin/bash

cargo --version
cargo build --release
./target/release/eetc-data-feed &