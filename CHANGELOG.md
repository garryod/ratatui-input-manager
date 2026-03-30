# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/garryod/ratatui-input-manager/compare/ratatui-input-manager-v0.2.0...ratatui-input-manager-v0.2.1) - 2026-03-30

### Added

- improve help widget readability
- add style setters to widgets

### Other

- fix badges
- add docs build to CI
- fix malformed docs
- add help widget example to readme
- clean up readme example
- add widget rendering tests
- setup dependabot
- add badges to the readme
- lint with clippy
- add a crossterm example

## [0.2.0](https://github.com/garryod/ratatui-input-manager/compare/ratatui-input-manager-v0.1.0...ratatui-input-manager-v0.2.0) - 2026-03-28

### Added

- add modifiers to widget hints
- add key modifier support

### Other

- group release-plz versions of crates
- factor out key press types
- add additional tests for modifier behaviour
- release v0.1.0

## [0.1.0](https://github.com/garryod/ratatui-input-manager/releases/tag/ratatui-input-manager-v0.1.0) - 2026-03-28

### Added

- add support for termwiz backend
- add support for termion backend
- make KeyMap & KeyBind generic over backend event types
- add block and style options to widgets
- combine keybind keys
- add help bar widget which renders keybinds in a single row
- add help widget which renders keybinds
- add metadata about which keys are abound
- add handler generation

### Other

- add necassary cargo metadata
- fix release-plz triggering
- fix readme example for non-tty environments
- setup release-plz for publishing
- add cargo check and test to workflows
- add cargo deny to workflows
- setup git cliff
- setup cargo-deny
- fix erroneous docstring for handle
- add usage examples to the help widgets
- add usage example to README
- fix typos
- add test for crossterm backend
