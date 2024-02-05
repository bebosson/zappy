#!/bin/sh
cargo run --bin server -- -n arsenal chelsea -c 1 -x 5 -y 5 -t 1 -p 1312 > /dev/pts/1 &
sleep 2; cargo run --bin server_gfx > /dev/pts/2 & 
sleep 2; cargo run --bin client -- arsenal test/arsenal.txt > /dev/pts/3 &
cargo run --bin client -- chelsea test/chelsea.txt > /dev/pts/4 &
