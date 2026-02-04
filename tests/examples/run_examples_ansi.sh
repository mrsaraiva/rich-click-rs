#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PY_SRC="/Volumes/Marcos/Arquivos/dev/mark/Proj/Libs/rich-click"
PY_DIR="$ROOT/tests/examples/python"
VENV="$PY_DIR/.venv"
OUT_DIR="$ROOT/tests/examples/output_ansi"

EXAMPLES=(
  "01_simple"
  "02_declarative"
  "03_groups_sorting"
  "04_rich_markup"
  "05_markdown"
  "06_arguments"
  "07_custom_errors"
  "08_metavars"
  "08_metavars_default"
  "09_envvar"
  "10_table_styles"
  "11_hello"
  "12_theme_simple"
)

mkdir -p "$PY_DIR" "$OUT_DIR/python" "$OUT_DIR/rust"

if [ ! -d "$VENV" ]; then
  python3 -m venv "$VENV"
fi

PIP_DISABLE_PIP_VERSION_CHECK=1 "$VENV/bin/pip" -q install "click>=8" "rich>=12" "typing-extensions>=4" >/dev/null

VERBOSE=0
if [[ "${1:-}" == "--verbose" ]]; then
  VERBOSE=1
fi

echo "Running example ANSI parity..."

for ex in "${EXAMPLES[@]}"; do
  echo "- $ex"
  RICH_CLICK_THEME='{"color_system":"standard"}' FORCE_COLOR=1 PY_COLORS=1 PYTHONPATH="$PY_SRC/src" \
    "$VENV/bin/python" "$PY_SRC/examples/${ex}.py" --help \
    > "$OUT_DIR/python/${ex}.ansi"
  RICH_CLICK_THEME='{"color_system":"standard"}' FORCE_COLOR=1 PY_COLORS=1 RUSTFLAGS="-Awarnings" \
    CARGO_TARGET_DIR="/tmp/rich-click-examples-target" \
    cargo run --quiet --example "$ex" -- --help > "$OUT_DIR/rust/${ex}.ansi"

  if ! cmp -s "$OUT_DIR/python/${ex}.ansi" "$OUT_DIR/rust/${ex}.ansi"; then
    if [[ "$VERBOSE" -eq 1 ]]; then
      echo "ANSI mismatch for ${ex} (raw bytes)"
      cmp -l "$OUT_DIR/python/${ex}.ansi" "$OUT_DIR/rust/${ex}.ansi" 2>/dev/null | head -n 20 || true
    fi

    # Normalize ANSI sequences (CSI/OSC/etc.) and compare again.
    python3 - <<'PY' "$OUT_DIR/python/${ex}.ansi" "$OUT_DIR/rust/${ex}.ansi" "$OUT_DIR/python/${ex}.norm" "$OUT_DIR/rust/${ex}.norm"
import re
import sys

CSI_RE = re.compile(rb"\x1b\[[0-?]*[ -/]*[@-~]")
OSC_RE = re.compile(rb"\x1b\].*?(?:\x07|\x1b\\)", re.DOTALL)
ESC_RE = re.compile(rb"\x1b[@-Z\\-_]")

def normalize(data: bytes) -> bytes:
    # Strip ANSI escape sequences entirely to compare visual text layout.
    data = OSC_RE.sub(b"", data)
    data = CSI_RE.sub(b"", data)
    data = ESC_RE.sub(b"", data)
    return data

py_in, rs_in, py_out, rs_out = sys.argv[1:]
py_data = open(py_in, "rb").read()
rs_data = open(rs_in, "rb").read()
open(py_out, "wb").write(normalize(py_data))
open(rs_out, "wb").write(normalize(rs_data))
PY

    if ! cmp -s "$OUT_DIR/python/${ex}.norm" "$OUT_DIR/rust/${ex}.norm"; then
      echo "ANSI mismatch for ${ex} (normalized)"
      if [[ "$VERBOSE" -eq 1 ]]; then
        cmp -l "$OUT_DIR/python/${ex}.norm" "$OUT_DIR/rust/${ex}.norm" 2>/dev/null | head -n 20 || true
      fi
      exit 1
    fi
    if [[ "$VERBOSE" -eq 1 ]]; then
      echo "ANSI normalized OK for ${ex}"
    fi
  fi
done

echo "ANSI examples OK (raw or normalized)"
