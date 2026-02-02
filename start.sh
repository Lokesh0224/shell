#!/bin/bash
# Change to script directory
cd "$(dirname "$0")"

# Build first
cargo build

# Run in new terminal
gnome-terminal -- bash -c "./target/debug/codecrafters-shell; exec bash"