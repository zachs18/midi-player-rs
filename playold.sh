#!/bin/bash
cargo b --release; ~/Programming/sound/midi.py "$1" | cargo r --release --bin player
