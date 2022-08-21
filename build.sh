#!/bin/bash

DIR="$(dirname "$0")"

RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build --release

cp "target/release/unsplash-rs" "unsplash-rs" 