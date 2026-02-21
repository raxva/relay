#!/bin/bash

cargo build --release && \
    ./target/release/raxva ./sample/sample.config.toml
