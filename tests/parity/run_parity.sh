#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
TARGET_DIR="/tmp/rich-click-parity-target"

phases=(phase1 phase2 phase3)

for PHASE in "${phases[@]}"; do
  PHASE_DIR="$ROOT_DIR/tests/parity/$PHASE"
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
  CARGO_TARGET_DIR="$TARGET_DIR" CARGO_BUILD_JOBS=1 cargo run --quiet > "$OUT_DIR/rust.txt"
  popd >/dev/null

  diff -u "$OUT_DIR/python.txt" "$OUT_DIR/rust.txt"

done

printf "Parity OK\n"
