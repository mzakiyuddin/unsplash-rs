#!/bin/bash

DIR="$(dirname "$0")"

cargo build --release

cp "$DIR/target/release/unsplash-rs" "$DIR/unsplash-rs"