#!/bin/bash

# Get the absolute path of the directory containing this script.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
# Change the working directory to the project root, which is one level up.
cd "$SCRIPT_DIR/.."

set -e

log() {
    echo "[RUNNER-INFO] $1"
}

# Create a timestamped directory for the test outputs
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
OUTPUT_DIR="test_outs/$TIMESTAMP"
mkdir -p "$OUTPUT_DIR"

log "Test outputs will be saved in: $OUTPUT_DIR"

log "Building frontend with wasm-pack..."
(cd hudwiz/frontend && wasm-pack build --target web --out-name wasm --out-dir ./dist --release)

log "Building server..."
cargo build --release -p server

log "Copying static assets..."
cp hudwiz/frontend/index.html hudwiz/frontend/dist/
mkdir -p hudwiz/frontend/dist/style
cp hudwiz/frontend/style/main.css hudwiz/frontend/dist/style/
# Copy the static directory which contains config.json
cp -r hudwiz/frontend/static hudwiz/frontend/dist/

log "Starting server in the background..."
# Run the already-compiled binary, redirecting its logs
./target/release/server > "$OUTPUT_DIR/server.log" 2>&1 &
SERVER_PID=$!
# Ensure server is killed on script exit, and ignore errors if it's already gone
trap 'log "Shutting down server..."; kill $SERVER_PID 2>/dev/null || true' EXIT

log "Waiting for server to start..."
sleep 5

log "Running Python verification script..."
# Set the log level for the python script, can be overridden from outside
export LOG_LEVEL=${LOG_LEVEL:-INFO}
log "Python script log level set to: $LOG_LEVEL"
python scripts/verify.py "$OUTPUT_DIR"

log "Verification process complete. Outputs are in $OUTPUT_DIR"
