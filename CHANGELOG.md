# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Changed

- Always print a newline at EOF.

## [0.4.2] - 2022-03-21

### Fixed

- Empty lines are now correctly handled when computing template indentation.

## [0.4.1] - 2022-02-14

## [0.4.0] - 2022-02-10

### Changed

- Block indentation width is now computed and adjusted according to the configuration.

### Fixed

- `indentTemplate` now works correctly when the content couldn't be
  formatted through dprint.

## [0.3.2] - 2022-02-03

### Added

- Plugin can now be updated through `dprint config update`.

## [0.3.1] - 2022-01-29

### Fixed

- Fixed missing indentation before block content when using `indentTemplate`.

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

[unreleased]: https://github.com/malobre/dprint-plugin-vue/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/malobre/dprint-plugin-vue/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/malobre/dprint-plugin-vue/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/malobre/dprint-plugin-vue/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/malobre/dprint-plugin-vue/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/malobre/dprint-plugin-vue/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/malobre/dprint-plugin-vue/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/malobre/dprint-plugin-vue/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/malobre/dprint-plugin-vue/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/malobre/dprint-plugin-vue/releases/tag/v0.1.0
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/index.html
