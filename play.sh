#!/bin/bash
cargo b --release; cargo r --release --bin parser "$1" | cargo r --release --bin player
