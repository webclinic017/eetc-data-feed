**EETC Data Feed**

Data Feed by EETC written in Rust.

**Description**:

Service for providing financial data via ZeroMQ.

**System requirements:**
- Linux (tested on Ubuntu 18.04 only)
- Rust 1.30.0 (https://www.rust-lang.org/tools/install)
- libzmq:
    - To install run these commands in your home dir:
    - `wget https://github.com/zeromq/libzmq/releases/download/v4.2.5/zeromq-4.2.5.tar.gz`
    (ref http://zeromq.org/intro:get-the-software)
    - `tar xvzf zeromq-4.2.5.tar.gz`
    - `sudo apt-get update`
    - `sudo apt-get install -y libtool pkg-config build-essential autoconf automake uuid-dev`
    - `cd zeromq-4.2.5`
    - `./configure`
    - `sudo make install`
    - `sudo ldconfig`
    - `ldconfig -p | grep zmq`
    - You should see something like:
        ```
        libzmq.so.5 (libc6,x86-64) => /usr/local/lib/libzmq.so.5
        libzmq.so (libc6,x86-64) => /usr/local/lib/libzmq.so
        ```

**Installation**:
- Download & extract the project & cd to project root directory
- Run command `cargo build` to build the project
- Run command `cargo run` to run the project

**Build project and run in production**:
- cd to project root dir & run command: `cargo build --release`
- Copy file _target/release/eetc-data-feed_ to server
- Run command:`./eetc-data-feed` to run it

**Usage**:
- First you have to get the snapshot of data to maintain(candles, order books, trades, etc.)
- Then you can subscribe to whichever topic you need to receive latest updates
- Request data snapshot for specific topic(s) - Python example:
```python
import zmq

context = zmq.Context()

req_socket = context.socket(zmq.REQ)
req_socket.connect("tcp://localhost:4445")

req_socket.send(b'candles:BTC/USD:1m')
message = req_socket.recv_multipart()
print(message[0].decode(), message[1].decode())

req_socket.send(b'trades:BTC/USD')
message = req_socket.recv_multipart()
print(message[0].decode(), message[1].decode())

req_socket.send(b'candles:BTC/USD:1h')
message = req_socket.recv_multipart()
print(message[0].decode(), message[1].decode())
# etc. you get the picture
```
- Subscribe to specific topic(s) - Python example:
```python
import zmq

context = zmq.Context()
socket = context.socket(zmq.SUB)

socket.connect("tcp://localhost:4444")
socket.setsockopt_string(zmq.SUBSCRIBE, 'candles:BTC/USD:1m')
socket.setsockopt_string(zmq.SUBSCRIBE, 'trades:BTC/USD')
socket.setsockopt_string(zmq.SUBSCRIBE, 'candles:ETH/USD:1h')
# etc. you get the picture

while True:
    multipart_msg = socket.recv_multipart()
    print(multipart_msg[0].decode(), multipart_msg[1].decode())
```
- Subscribe to all topics - Python example:
```python
import zmq

context = zmq.Context()
socket = context.socket(zmq.SUB)

socket.connect("tcp://localhost:4444")
socket.setsockopt_string(zmq.SUBSCRIBE, '')

while True:
    multipart_msg = socket.recv_multipart()
    print(multipart_msg[0].decode(), multipart_msg[1].decode())
```
- All data is in JSON format(for now)

**Topics**:
- Topics can have these formats:
    - `candles:<<symbol>>/USD:<<timeframe>>`
    - `trades:<<symbol>>/USD`
    - `book:<<symbol>>/USD`
    - `ticker:<<symbol>>/USD`
- Values for `<<symbol>>`:
    - `BTC`
    - `XRP`
    - `ETH`
    - `IOT`
    - `LTC`
    - `EOS`
    - `ETC`
    - `XMR`
    - `NEO`
    - `ZEC`
    - `BAB`
    - `BSV`
- Values for `<<timeframe>>`:
    - `1m`
    - `5m`
    - `15m`
    - `30m`
    - `1h`
    - `3h`
    - `6h`
    - `12h`
    - `1D`
    - `7D`
    - `14D`
    - `1M`

**Licence**:
- https://github.com/delicmakaveli/eetc-data-feed/blob/master/LICENSE

