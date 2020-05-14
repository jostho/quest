# Quest

This is an quiz program written in rust.

## Environment

* fedora 32
* rust 1.43
* make 4.2

## Build

To build or run, use cargo

    cargo build
    cargo run

## Run

Generate content

    ./target/debug/quest -g -i /usr/share/iso-codes/json/iso_3166-1.json

Ask the quiz

    ./target/debug/quest -i output.csv
