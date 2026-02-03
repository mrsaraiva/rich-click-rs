# rich-click-rs Development Roadmap

A phased plan to port Python rich-click to Rust, targeting feature parity with Click 8.3.1 + rich-click.
Reference: `/Volumes/Marcos/Arquivos/dev/mark/Proj/Libs/rich-click`.

**Last Updated:** 2026-02-03

**Project State:** Phase 0 scaffold complete (crate + basic rich help renderer).

**Goal:** Full feature parity with Python rich-click’s help rendering (themes, panels, option tables, command tables, markup/markdown, errors).

---

## Phase 0: Scaffold & Integration

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Done | Create `rich-click-rs` crate, basic module layout | N/A | `src/config.rs`, `src/render.rs`, `src/lib.rs` |
| Done | Local deps to `click-rs` + `rich-rs` | N/A | Path dependencies |
| Done | Minimal RichHelp config struct | `rich_click.py` | Subset of config |
| Done | Basic RichHelp renderer (usage, help, panels, tables) | `rich_help_formatter.py`, `rich_help_rendering.py` | Initial output only |
| Done | `main_rich_command` / `main_rich_group` wrappers | `rich_click.py` | Mirrors eager help/version path |

---

## Phase 1: Configuration & Theme System

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Todo | Port `RichClickTheme` model | `rich_click_theme.py` | Theme combine + defaults |
| Todo | Port `RichHelpConfiguration` loader | `rich_help_configuration.py` | Globals + overrides |
| Todo | Theme resolution via env var | `rich_click.py` | `ENABLE_THEME_ENV_VAR`, `THEME` |
| Todo | Merge theme defaults into config | `rich_click.py` | Apply theme -> config styles |
| Todo | Expose theme list + CLI hook | `cli.py` | `rich-click --themes` parity |

---

## Phase 2: Help Rendering Core

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Partial | Rich usage line (styled segments) | `rich_help_rendering.py` | Currently plain Text + style |
| Partial | Help text / deprecation handling | `rich_help_rendering.py` | Needs per-style segments |
| Todo | Rich markup + markdown parsing options | `rich_help_formatter.py` | `USE_MARKDOWN`, `USE_RICH_MARKUP` |
| Todo | Help text emoji support | `rich_help_formatter.py` | `TEXT_EMOJIS` |
| Todo | Inline help aliases | `rich_help_rendering.py` | `HELPTEXT_ALIASES_STRING` |
| Todo | Width/max-width handling parity | `rich_help_formatter.py` | Console sizing/override |

---

## Phase 3: Panels & Tables

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Partial | Options panel + table | `rich_panel.py`, `rich_help_rendering.py` | Basic table in panel |
| Partial | Commands panel + table | `rich_panel.py`, `rich_help_rendering.py` | Basic command listing |
| Partial | Arguments panel + table | `rich_panel.py`, `rich_help_rendering.py` | Uses parameter help |
| Todo | Per-panel styles + box types | `rich_click.py` | Full style matrix |
| Todo | Column types & width ratios | `rich_help_configuration.py` | `*_COLUMN_TYPES` |
| Todo | Panel title padding & inline help | `rich_click.py` | `PANEL_INLINE_HELP_IN_TITLE` |
| Todo | Row styles, borders, and line settings | `rich_click.py` | `STYLE_*_TABLE_*` |

---

## Phase 4: Option/Argument Metadata Rendering

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Partial | Envvar/default/required display | `rich_parameter.py` | Currently raw text |
| Todo | Style spans for metavar/default/envvar | `rich_help_rendering.py` | Apply rich styles |
| Todo | Range & metavars append formatting | `rich_help_rendering.py` | `APPEND_*_HELP_STRING` |
| Todo | Option group support | `rich_click.py` | `OPTION_GROUPS` |
| Todo | Command group support | `rich_click.py` | `COMMAND_GROUPS` |
| Todo | Alias rendering | `rich_group.py` | `HELPTEXT_SHOW_ALIASES` |

---

## Phase 5: Error Rendering & Suggestions

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Todo | Rich error panel rendering | `rich_help_formatter.py` | Errors panel + padding |
| Todo | Suggestion rendering | `rich_click.py` | `ERRORS_SUGGESTION*` |
| Todo | Abort message rendering | `rich_click.py` | `ABORTED_TEXT` |

---

## Phase 6: CLI Integration & Public API

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Partial | Public RichHelp trait | `decorators.py` | Expose get_rich_help |
| Todo | Decorator/macros parity | `decorators.py` | Rust-friendly builder helpers |
| Todo | CLI wrapper for theme listing | `cli.py` | `rich-click` entry point |
| Todo | Documentation examples | `README.md`, docs | Usage + theming |

---

## Phase 7: Parity Tests

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Todo | Python parity scripts | `tests/` | Reference outputs |
| Todo | Rust parity binaries | N/A | Compare with Python output |
| Todo | `tests/parity/run_parity.sh` | click-rs model | Diff-based comparison |

---

## Phase 8: Release Readiness

| Status | Task | Python Reference | Notes |
|--------|------|------------------|-------|
| Todo | MSRV confirmation | N/A | Align with click-rs/rich-rs |
| Todo | crates.io metadata | N/A | License, keywords, categories |
| Todo | 1.0 API stabilization | N/A | Versioning + changelog |

---

## Notes

- Parity scope includes configuration flags from `rich_click.py` and help rendering logic in `rich_help_formatter.py` / `rich_help_rendering.py`.
- Rust implementation should follow the same diff-based parity testing approach used in `click-rs` and `rich-rs`.
- Some Python features (dynamic environment/global config) may require Rust-specific adaptation (builder patterns, default globals, or macro helpers).
