# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/garryod/ratatui-input-manager/compare/ratatui-input-manager-derive-v0.2.1...ratatui-input-manager-derive-v0.3.0) - 2026-03-30

### Added

- [**breaking**] require doc comments on keybinds

### Other

- release v0.2.1
- fix badges

## [0.2.1](https://github.com/garryod/ratatui-input-manager/compare/ratatui-input-manager-derive-v0.2.0...ratatui-input-manager-derive-v0.2.1) - 2026-03-30

### Other

- fix badges

## [0.2.0](https://github.com/garryod/ratatui-input-manager/compare/ratatui-input-manager-derive-v0.1.0...ratatui-input-manager-derive-v0.2.0) - 2026-03-28

### Added

- add key modifier support

### Other

- factor out key press types
- release v0.1.0

## [0.1.0](https://github.com/garryod/ratatui-input-manager/releases/tag/ratatui-input-manager-derive-v0.1.0) - 2026-03-28

### Added

- add support for termwiz backend
- add support for termion backend
- make KeyMap & KeyBind generic over backend event types
- combine keybind keys
- add metadata about which keys are abound
- add handler generation

### Other

- add necassary cargo metadata
- fix readme example for non-tty environments
- setup cargo-deny
- add usage example to README
