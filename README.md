# Quest

![CI](https://github.com/jostho/quest/workflows/CI/badge.svg)

This is a capital quiz program written in rust.

## Environment

* fedora 36
* rust 1.61

## Build

To build, use `cargo`

    cargo build

## Run

Get `countries.json` from [here](https://github.com/mledoze/countries)

    curl -O https://raw.githubusercontent.com/mledoze/countries/master/dist/countries.json

Generate csv content

    ./target/debug/quest -g -i countries.json

Ask the quiz

    ./target/debug/quest -i countries.csv
