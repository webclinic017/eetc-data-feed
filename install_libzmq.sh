#!/usr/bin/bash

############################################################
# Copy this script to your home directory and run it
############################################################

cd /home/ubuntu/

# Download zeromq
# Ref http://zeromq.org/intro:get-the-software
wget https://github.com/zeromq/libzmq/releases/download/v4.2.5/zeromq-4.2.5.tar.gz

# Unpack tarball package
tar xvzf zeromq-4.2.5.tar.gz

# Install dependency
sudo apt-get update && \
sudo apt-get install -y libtool pkg-config build-essential autoconf automake uuid-dev

# Create make file
cd zeromq-4.2.5
./configure

# Build and install(root permission only)
sudo make install

# Install zeromq driver on linux
sudo ldconfig

# Check installed
ldconfig -p | grep zmq

# Expected
############################################################
# libzmq.so.5 (libc6,x86-64) => /usr/local/lib/libzmq.so.5
# libzmq.so (libc6,x86-64) => /usr/local/lib/libzmq.so
############################################################
