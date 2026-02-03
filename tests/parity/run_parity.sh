#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
PHASE_DIR="$ROOT_DIR/tests/parity/phase1"
PY_DIR="$PHASE_DIR/python"
RS_DIR="$PHASE_DIR/rust"
OUT_DIR="$PHASE_DIR/output"

mkdir -p "$OUT_DIR"

if [ ! -d "$PY_DIR/.venv" ]; then
  python3 -m venv "$PY_DIR/.venv"
fi

"$PY_DIR/.venv/bin/pip" install -r "$ROOT_DIR/tests/parity/requirements.txt" >/dev/null

"$PY_DIR/.venv/bin/python" "$PY_DIR/test_help.py" > "$OUT_DIR/python.txt"

pushd "$RS_DIR" >/dev/null
cargo run --quiet > "$OUT_DIR/rust.txt"
popd >/dev/null

diff -u "$OUT_DIR/python.txt" "$OUT_DIR/rust.txt"

printf "Parity OK\n"
