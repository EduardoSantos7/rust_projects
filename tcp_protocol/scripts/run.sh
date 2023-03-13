#!/bin/bash
cargo b --release
ls $CARGO_TARGET_DIR
sudo setcap cap_net_admin=eip target/release/tcp_protocol
target/release/tcp_protocol &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid