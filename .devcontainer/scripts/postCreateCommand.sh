#!/bin/bash

# Check if sea-orm-cli is already installed
if ! command -v sea-orm-cli >/dev/null 2>&1; then
  echo "sea-orm-cli is not installed. Installing..."
  cargo install sea-orm-cli
else
  echo "sea-orm-cli is already installed."
fi
