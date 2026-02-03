#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PY_SRC="/Volumes/Marcos/Arquivos/dev/mark/Proj/Libs/rich-click"
PY_DIR="$ROOT/tests/examples/python"
VENV="$PY_DIR/.venv"
OUT_DIR="$ROOT/tests/examples/output"

EXAMPLES=(
  "01_simple"
  "02_declarative"
  "11_hello"
)

mkdir -p "$PY_DIR" "$OUT_DIR/python" "$OUT_DIR/rust"

if [ ! -d "$VENV" ]; then
  python3 -m venv "$VENV"
fi

PIP_DISABLE_PIP_VERSION_CHECK=1 "$VENV/bin/pip" -q install "click>=8" "rich>=12" "typing-extensions>=4" >/dev/null

echo "Running example help parity..."

for ex in "${EXAMPLES[@]}"; do
  echo "- $ex"
  PYTHONPATH="$PY_SRC/src" "$VENV/bin/python" "$PY_SRC/examples/${ex}.py" --help > "$OUT_DIR/python/${ex}.txt"
  RUSTFLAGS="-Awarnings" CARGO_TARGET_DIR="/tmp/rich-click-examples-target" \
    cargo run --quiet --example "$ex" -- --help > "$OUT_DIR/rust/${ex}.txt"

  if ! diff -u "$OUT_DIR/python/${ex}.txt" "$OUT_DIR/rust/${ex}.txt" >/dev/null; then
    echo "Mismatch for ${ex}"
    diff -u "$OUT_DIR/python/${ex}.txt" "$OUT_DIR/rust/${ex}.txt"
    exit 1
  fi
done

echo "Examples OK"
