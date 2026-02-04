# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-02-04

### Added
- Core rich-click scaffolding and public entry points (`main_rich_command`, `main_rich_group`).
- Theme system with config loader, defaults, env var support, and theme listing.
- Rich help rendering: usage, help text, panels (options/commands/arguments), and tables.
- Rich markup/markdown rendering options with paragraph linebreak controls.
- Option/command grouping and metadata sections (envvar/default/required/metavar).
- Alias metadata support in help output.
- Rich error rendering and helper entry points.
- Example suite parity runner plus ANSI parity runner (normalized for visual diffs).
- Gap inventory documentation for tracking parity issues.

### Fixed
- Help text styling and spacing aligned with Python rich-click.
- Table column sizing, padding, and panel title spacing aligned with Python rich-click.
- Metavar/default/help rendering parity for options and arguments.
- Slim theme layout parity (compact list format and default formatting).

### Tests
- Phase 1–3 parity harness for Python rich-click comparisons.
- Example parity checks for Python vs Rust outputs.
