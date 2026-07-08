#!/usr/bin/env bash
set -euo pipefail

cargo metadata --format-version 1 --no-deps >/dev/null
cargo test --workspace
bash scripts/check_no_placeholders.sh
python3 scripts/check_line_count.py
