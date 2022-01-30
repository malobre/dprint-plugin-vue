# dprint-plugin-vue

Format Vue SFC.

This plugin only parses root-level blocks, extract their content, and format
them through `dprint` according to the `lang` attribute or a default:

| Block      | Default |
| ---------- | ------- |
| `script`   | `js`    |
| `template` | `html`  |
| `style`    | `css`   |

This means you also need to install plugins for languages you want to format,
such as
[`dprint-plugin-typescript`](https://github.com/dprint/dprint-plugin-typescript)
for JavaScript / TypeScript.

## Usage

See the [GitHub Releases](https://github.com/malobre/dprint-plugin-vue/releases).

## Configuration

| Key              | Default | Description                                |
| ---------------- | ------- | ------------------------------------------ |
| `indentTemplate` | `true`  | Indent the content of the `<template>` tag |
| `indentWidth`    | `2`     | Width of the indentation                   |
| `useTabs`        | `false` | Use tabs for indentation                   |
