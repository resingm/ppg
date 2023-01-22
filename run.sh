#!/bin/bash
#
#

release_bin="$WORKSPACE/de/maxresing/ppg/target/release/ppg"
tun="tun0"
tip="192.168.0.1/24"

cargo build --release
sudo setcap cap_net_admin=eip $release_bin

$release_bin -i $tun &
pid=$!
sudo ip addr add $tip dev $tun
sudo ip link set up dev $tun

echo "raw-packets"
echo "========================="
echo "  PID:          $pid"
echo "  Interface:    $tun"
echo "  Interface IP: $tip"
echo "========================="


trap "kill $pid" INT TERM
wait $pid

