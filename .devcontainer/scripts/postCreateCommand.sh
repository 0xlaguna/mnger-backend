#!/bin/bash

# Check if diesel_cli is already installed
if ! command -v diesel >/dev/null 2>&1; then
  echo "diesel_cli is not installed. Installing..."
  cargo install diesel_cli --no-default-features --features postgres
else
  echo "diesel_cli is already installed."
fi
