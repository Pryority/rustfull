#!/bin/bash

# Start backend in the background
cd backend
cargo watch -q -c -w ./src/ -x run &
cd ..

# Start frontend in the background
cd frontend
trunk serve --port 3000 &
cd ..

# Wait for both processes to finish
wait
