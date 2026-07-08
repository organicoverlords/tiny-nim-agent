#!/usr/bin/env bash
set -euo pipefail

paths=(Cargo.toml crates apps)

if grep -RInE 'todo!\(|unimplemented!\(|not implemented|placeholder|fake' \
  --include='*.rs' \
  --include='Cargo.toml' \
  "${paths[@]}"; then
  echo "Placeholder text found in product source"
  exit 1
fi

echo "No placeholder text found in product source"
