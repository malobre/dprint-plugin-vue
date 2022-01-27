# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2022-01-27

### Added

- Added configuration:

  | Key              | Default | Description                                |
  | ---------------- | ------- | ------------------------------------------ |
  | `indentTemplate` | `true`  | Indent the content of the `<template>` tag |
  | `indentWidth`    | `2`     | Width of the indentation                   |
  | `useTabs`        | `false` | Use tabs for indentation                   |

### Changed

- Formatted content is now end-trimmed.
- Block tag names are now compared case-insensitively.
- Block content is passed to dprint with `file.vue.{lang}` as its filename.

## [0.2.0] - 2022-01-25

### Added

- Some documentation.
- Basic README.
- Latest download URL in the README.
- CHANGELOG.

### Removed

- Removed `Configuration`.

## [0.1.1] - 2022-01-25

### Fixed

- Fix nested tags parsing (#1).

## [0.1.0] - 2022-01-25

### Added

- Initial code

[unreleased]: https://github.com/malobre/dprint-plugin-vue/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/malobre/dprint-plugin-vue/compare/v0.1.0...v0.3.0
[0.2.0]: https://github.com/malobre/dprint-plugin-vue/compare/v0.1.0...v0.2.0
[0.1.1]: https://github.com/malobre/dprint-plugin-vue/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/malobre/dprint-plugin-vue/releases/tag/v0.1.0
