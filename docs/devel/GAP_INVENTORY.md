# Gap Inventory (rich-click-rs vs Python rich-click)

This document tracks **known behavior/output gaps** between `rich-click-rs` and **Python rich-click**.
It is meant to be living documentation: every intentional divergence should be recorded here,
and every unintentional divergence should be turned into a tracked task.

## Reference Baseline

- **Python rich-click:** latest pip install at parity run time (`tests/parity/requirements.txt`).
- **Parity runner:** `tests/parity/run_parity.sh` builds Rust output and compares against Python output.

## Status Keys

- **Open:** known gap, not yet scheduled/implemented.
- **Planned:** work item exists in `docs/devel/ROADMAP.md`.
- **Done:** gap closed (or explicitly accepted as “by design”).
- **By Design:** intentional divergence (documented rationale).

## Inventory

| ID | Area | Gap | Impact | Status | Notes / Next Steps |
|----|------|-----|--------|--------|--------------------|
| GAP-HELP-001 | Metavars | Option metavars default to `TEXT` in Rust; Python shows type names (e.g. `INTEGER`) and omits metavars for bool flags. | Medium (help clarity) | Open | Use click-rs param/type info to derive metavars; suppress metavar for bool flag variants. |
| GAP-HELP-002 | Slim theme | Rust uses table rendering for slim theme; Python uses a compact list layout and `default=...` formatting. | Low | Open | Add slim-specific renderer and default formatting parity (`default=1`, metavar bracketing). |
| GAP-HELP-003 | Spacing | Rust output has extra blank lines and extra spaces in panel titles (e.g. `╭─  Options  ─`). | Low | Open | Align panel title padding and section spacing with Python output. |
| GAP-HELP-004 | Aliases | Rust shows alias list in command table by default; Python does not show aliases unless configured. | Low | Open | Gate alias rendering behind config flag (`HELPTEXT_SHOW_ALIASES`). |
| GAP-HELP-005 | Option table columns | Column widths/alignment differ; required marker and spacing differ from Python. | Low | Open | Add column width config parity and required marker formatting. |

## Adding a New Gap

1. Add a new row with a unique `GAP-<AREA>-NNN` ID.
2. Include a minimal reproduction if possible (test name or parity phase/module).
3. If it’s intentional, mark **By Design** and write a short rationale in the Notes column.
