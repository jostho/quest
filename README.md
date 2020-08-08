# Quest

![CI](https://github.com/jostho/quest/workflows/CI/badge.svg)

This is a capital quiz program written in rust.

## Environment

* fedora 32
* rust 1.43

## Build

To build or run, use cargo

    cargo build
    cargo run

## Run

Get `countries.json` from [here](https://github.com/mledoze/countries)

    curl -O https://raw.githubusercontent.com/mledoze/countries/master/dist/countries.json

Generate csv content

    ./target/debug/quest -g -i countries.json

Ask the quiz

    ./target/debug/quest -i countries.csv
