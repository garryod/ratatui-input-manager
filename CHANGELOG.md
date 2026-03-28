# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
